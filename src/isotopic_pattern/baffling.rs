//! An implementation of the Baffling Recursive Algorithm for Isotopic distributioN (BRAIN)
//! originally published in [Dittwald, 2013](http://dx.doi.org/10.1021/ac303439m).
use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::element::Element;
use crate::isotopic_pattern::{Peak, PeakList, poisson_approximate_n_peaks_of};
use crate::{mass_charge_ratio, ChemicalComposition, ElementSpecification};

use fnv::FnvBuildHasher as RandomState;

pub type DVec = Vec<f64>;

#[derive(Debug, Default, Clone)]
pub struct ElementRegistry {
    store: HashMap<String, usize, RandomState>,
    counter: usize,
}


impl<'element> ElementRegistry {
    pub fn new(store: HashMap<String, usize, RandomState>) -> Self {
        let counter = if store.len() > 0 {
            store.values().max().unwrap() + 1
        } else {
            0
        };

        Self { store, counter }
    }

    pub fn get(&self, element_name: &str) -> Option<usize> {
        self.store.get(element_name).and_then(|v| {
            Some(*v)
        })
    }

    pub fn register(&mut self, element_name: &str) -> usize {
        if let Some(val) = self.store.get(element_name) {
            return *val
        }

        let val = self.counter;
        self.store.insert(element_name.to_string(), val);
        self.counter += 1;
        val
    }
}



#[derive(Debug, Clone, Copy)]
pub struct ElementKey<'a>{
    pub element: &'a Element,
    pub key: usize
}


#[derive(Debug, Clone)]
pub struct PolynomialParameters {
    elementary_symmetric_polynomial: DVec,
    power_sum: DVec,
}

pub fn vietes(coefficients: &DVec) -> DVec {
    let n = coefficients.len();
    let mut esp = DVec::with_capacity(n);
    let tail = coefficients[n - 1];
    for i in 0..n {
        let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
        let el = sign * coefficients[n - i - 1] / tail;
        esp.push(el);
    }
    return esp;
}

impl PolynomialParameters {
    pub fn update_power_sum(&mut self) {
        let begin = self.power_sum.len();
        let end = self.elementary_symmetric_polynomial.len();

        for k in begin..end {
            if k == 0 {
                self.power_sum.push(0.0);
                continue;
            }
            let mut temp_ps = 0.0;
            let mut sign = -1.0;
            for j in 1..k {
                sign *= -1.0;
                temp_ps += sign * self.elementary_symmetric_polynomial[j] * self.power_sum[k - j];
            }
            sign *= -1.0;
            temp_ps += sign * self.elementary_symmetric_polynomial[k] * (k as f64);
            self.power_sum.push(temp_ps);
        }
    }

    pub fn update_elementary_symmetric_polynomial(&mut self, order: i32) {
        let begin = self.elementary_symmetric_polynomial.len();
        let end = self.power_sum.len();
        self.elementary_symmetric_polynomial.reserve(end.saturating_sub(begin));
        for k in begin..end {
            if k == 0 {
                self.elementary_symmetric_polynomial.push(1.0);
            } else if k > (order as usize) {
                self.elementary_symmetric_polynomial.push(0.0);
            } else {
                let el = (1..k + 1).into_iter().map(|j| {
                    let sign = if (j % 2) == 1 { 1.0 } else { -1.0 };
                    return sign * self.power_sum[j] * self.elementary_symmetric_polynomial[k - j]
                }).sum::<f64>() / k as f64;
                self.elementary_symmetric_polynomial.push(el);
            }
        }
    }

    pub fn newton_optimization(&mut self, order: i32) {
        let psn = self.power_sum.len();
        let espn = self.elementary_symmetric_polynomial.len();
        if psn > espn {
            self.update_elementary_symmetric_polynomial(order);
        } else if psn < espn {
            self.update_power_sum();
        }
    }

    pub fn isotopic_coefficients(element: &Element, with_mass: bool, accumulator: &mut DVec) {
        let max_isotope_number = element.max_neutron_shift;
        let min_neutron_shift = element.min_neutron_shift;
        let monoisotopic_number = element.element_number as usize;
        let n = element.isotopes.len();

        for z in min_neutron_shift..max_isotope_number + 1 {
            let i = (z - min_neutron_shift) as usize;
            let k = (n + monoisotopic_number - i - 1) as u16;
            // let isotope = match element.isotope_by_shift(&k) {
            let isotope = match element.isotopes.get(&k) {
                Some(isotope) => isotope,
                None => {
                    continue;
                }
            };
            let current_order = (max_isotope_number - isotope.neutron_shift) as usize;
            let coef = if with_mass { isotope.mass } else { 1.0 };
            if current_order > accumulator.len() {
                for _j in accumulator.len()..(current_order) {
                    accumulator.push(0.0);
                }
                accumulator.push(coef * isotope.abundance);
            } else if current_order == accumulator.len() {
                accumulator.push(coef * isotope.abundance);
            } else {
                panic!("Error! Unordered isotopes for {}", element.symbol);
            }
        }
    }

    pub fn from_element(
        element: &Element,
        with_mass: bool,
        accumulator: &mut DVec,
    ) -> PolynomialParameters {
        let n = element.max_neutron_shift;
        accumulator.reserve(n as usize);
        PolynomialParameters::isotopic_coefficients(element, with_mass, accumulator);

        let elementary_symmetric_polynomial = vietes(accumulator);
        let power_sum = DVec::with_capacity(elementary_symmetric_polynomial.len() + 4);
        let order = accumulator.len() - 1;
        let mut result = PolynomialParameters {
            elementary_symmetric_polynomial,
            power_sum,
        };
        result.newton_optimization(order as i32);
        return result;
    }
}

#[derive(Debug, Clone)]
pub struct PhiConstants {
    pub order: i32,
    pub element_key: String,
    pub element_coefficients: PolynomialParameters,
    pub mass_coefficients: PolynomialParameters,
}

impl PhiConstants {
    pub fn from_element(element: &Element) -> PhiConstants {
        let mut accumulator = DVec::new();
        let order = element.max_neutron_shift as i32;
        let element_coefficients =
            PolynomialParameters::from_element(element, false, &mut accumulator);
        accumulator.clear();
        let mass_coefficients = PolynomialParameters::from_element(element, true, &mut accumulator);
        return PhiConstants {
            element_key: element.symbol.clone(),
            order,
            element_coefficients,
            mass_coefficients,
        };
    }

    pub fn from_element_key(element_key: &ElementKey) -> PhiConstants {
        let mut accumulator = DVec::new();
        let order = element_key.element.max_neutron_shift as i32;
        let element_coefficients =
            PolynomialParameters::from_element(element_key.element, false, &mut accumulator);
        accumulator.clear();
        let mass_coefficients = PolynomialParameters::from_element(element_key.element, true, &mut accumulator);
        return PhiConstants {
            element_key: element_key.element.symbol.clone(),
            order,
            element_coefficients,
            mass_coefficients,
        };
    }
}


pub type PhiKey = str;

#[derive(Debug, Clone)]
pub struct IsotopicConstants<'lifespan> {
    // pub constants: HashMap<&'lifespan PhiKey, PhiConstants, RandomState>,
    pub constants: Vec<(&'lifespan PhiKey, PhiConstants)>,
    pub order: i32,
}

impl<'lifespan, 'outer: 'lifespan> IsotopicConstants<'lifespan> {
    pub fn new(size: usize) -> IsotopicConstants<'lifespan> {
        IsotopicConstants {
            // constants: HashMap::with_capacity_and_hasher(size, RandomState::default()),
            constants: Vec::with_capacity(size),
            order: 0,
        }
    }

    pub fn get(&self, symbol: &PhiKey) -> Option<&PhiConstants> {
        // self.constants.get(symbol)
        self.constants.iter().find(|(k, _)| {
            *k == symbol
        }).and_then(|(_, v)| {
            Some(v)
        })
    }

    pub fn set(&mut self, symbol: &'lifespan PhiKey, constants: PhiConstants) {
        // self.constants.insert(symbol, constants);
        self.constants.push((symbol, constants))
    }

    pub fn add(&mut self, element: &'outer Element) {
        match self.get(element.symbol.as_ref()) {
            Some(_c) => return,
            None => {}
        };

        let phi = PhiConstants::from_element(element);
        // self.constants.insert(element.symbol.as_ref(), phi);
        self.set(element.symbol.as_ref(), phi);
    }

    pub fn add_key(&mut self, element_key: &'outer ElementKey) {
        self.add(element_key.element)
    }

    pub fn update(&mut self) {
        for (_symbol, elt_params) in self.constants.iter_mut() {
            if self.order < elt_params.order {
                continue;
            }

            (elt_params.order..self.order + 1).for_each(|_| {
                elt_params
                    .element_coefficients
                    .elementary_symmetric_polynomial
                    .push(0.0);
                elt_params
                    .mass_coefficients
                    .elementary_symmetric_polynomial
                    .push(0.0);
            });

            elt_params.order = elt_params
                .element_coefficients
                .elementary_symmetric_polynomial
                .len() as i32;
            elt_params
                .element_coefficients
                .newton_optimization(elt_params.order);
            elt_params
                .mass_coefficients
                .newton_optimization(elt_params.order);
        }
    }

    pub fn nth_element_power_sum(&self, symbol: &PhiKey, order: usize) -> f64 {
        let phi = self
            .get(symbol)
            .unwrap_or_else(|| panic!("Expected element {} in constants", symbol));
        phi.element_coefficients.power_sum[order]
    }

    pub fn nth_element_power_sum_mass(&self, symbol: &str, order: usize) -> f64 {
        let phi = self
            .get(symbol)
            .unwrap_or_else(|| panic!("Expected element {} in constants", symbol));
        phi.mass_coefficients.power_sum[order]
    }
}

#[derive(Debug, Clone)]
pub struct IsotopicConstantsCache<'lifespan> {
    pub cache: HashMap<&'lifespan PhiKey, PhiConstants, RandomState>,
}

impl<'lifespan, 'outer: 'lifespan> IsotopicConstantsCache<'lifespan> {
    pub fn new() -> IsotopicConstantsCache<'lifespan> {
        return IsotopicConstantsCache {
            cache: HashMap::with_capacity_and_hasher(6, RandomState::default()),
        };
    }

    pub fn checkout(&mut self, symbol: &PhiKey) -> Option<PhiConstants> {
        self.cache.remove(symbol)
    }

    pub fn receive(&mut self, symbol: &'lifespan PhiKey, constants: PhiConstants) -> bool {
        let entry = self.cache.entry(symbol);
        match entry {
            Entry::Vacant(ent) => {
                ent.insert(constants);
                true
            }
            Entry::Occupied(mut ent) => {
                if ent.get().order > constants.order {
                    false
                } else {
                    ent.insert(constants);
                    true
                }
            }
        }
    }

    pub fn receive_from(&mut self, mut params: IsotopicConstants<'lifespan>) {
        for (k, v) in params.constants.drain(..) {
            self.receive(k, v);
        }
    }
}

pub fn max_variants(composition: &ChemicalComposition) -> i32 {
    let acc = composition
        .iter()
        .map(|(elt, cnt)| elt.element.max_neutron_shift as i32 * *cnt)
        .sum();
    acc
}

pub fn guess_npeaks(composition: &ChemicalComposition, max_npeaks: i32) -> i32 {
    // let total_variants = max_variants(composition);
    // let npeaks = (total_variants as f64).sqrt() as i32 - 2;
    // let result = cmp::min(cmp::max(npeaks, 3), max_npeaks);
    let result = poisson_approximate_n_peaks_of(composition.mass(), 0.9999) as i32;
    return result.min(max_npeaks);
}

struct ElementPolynomialMap<'a> {
    pub polynomials: Vec<(&'a str, DVec)>
}

impl<'a> ElementPolynomialMap<'a> {
    pub fn new(size: usize) -> ElementPolynomialMap<'a> {
        ElementPolynomialMap {
            polynomials: Vec::with_capacity(size)
        }
    }

    pub fn set(&mut self, symbol: &'a str, polynomial: DVec) {
        self.polynomials.push((symbol, polynomial));
    }

    pub fn get(&self, symbol: &'a str) -> &DVec {
        &self.polynomials.iter().find(|(k, _)| {
            *k == symbol
        }).unwrap().1
    }
}

#[derive(Debug)]
pub struct IsotopicDistribution<'lifespan, 'outer> {
    pub composition: ChemicalComposition<'outer>,
    pub constants: IsotopicConstants<'lifespan>,
    pub order: i32,
    pub average_mass: f64,
    pub monoisotopic_peak: Peak,
    pub max_variants: i32,
}

impl<'lifespan: 'transient, 'transient, 'outer: 'lifespan> IsotopicDistribution<'lifespan, 'outer> {
    pub fn from_composition(
        composition: ChemicalComposition<'lifespan>,
        order: i32,
    ) -> IsotopicDistribution<'lifespan, 'lifespan> {
        let mut inst = IsotopicDistribution::fill_from_composition(composition, order);
        inst.populate_constants();
        inst
    }

    fn fill_from_composition(
        composition: ChemicalComposition<'outer>,
        order: i32,
    ) -> IsotopicDistribution<'lifespan, 'outer> {
        let mut inst = IsotopicDistribution {
            constants: IsotopicConstants::new(composition.len()),
            max_variants: max_variants(&composition),
            composition,
            order: 0,
            average_mass: 0.0,
            monoisotopic_peak: Peak {
                mz: 0.0,
                intensity: 0.0,
                charge: 0,
            },
        };
        inst.update_order(order);
        inst.monoisotopic_peak = inst.make_monoisotopic_peak();
        inst
    }

    pub fn from_composition_and_cache(
        composition: ChemicalComposition<'outer>,
        order: i32,
        cache: &'transient mut IsotopicConstantsCache<'outer>,
    ) -> IsotopicDistribution<'lifespan, 'outer> {
        let mut inst = IsotopicDistribution::fill_from_composition(composition, order);
        inst.populate_constants_from_cache(cache);
        inst
    }

    fn populate_constants_from_cache(
        &mut self,
        cache: &'transient mut IsotopicConstantsCache<'outer>,
    ) {
        for (elt, _cnt) in self.composition.iter() {
            match cache.checkout(&elt.element.symbol) {
                None => {
                    self.constants.add(elt.element);
                }
                Some(isoconst) => {
                    self.constants.set(&elt.element.symbol, isoconst);
                }
            };
        }
        self.constants.update();
    }

    fn populate_constants(&mut self) {
        for (elt, _cnt) in self.composition.iter() {
            self.constants.add(elt.element);
        }
        self.constants.update();
    }

    fn make_monoisotopic_peak(&self) -> Peak {
        let mz = self.composition.mass();
        let mut intensity = 0.0;
        for (elt, _cnt) in self.composition.iter() {
            let element = elt.element;
            intensity += element.isotopes[&element.most_abundant_isotope]
                .abundance
                .ln();
        }
        intensity = intensity.exp();
        return Peak {
            mz,
            intensity: intensity,
            charge: 0,
        };
    }

    pub fn update_order(&mut self, order: i32) {
        if order == -1 {
            self.order = self.max_variants;
        } else {
            self.order = cmp::min(order, self.max_variants);
        }
        self.constants.order = self.order;
    }

    pub fn phi_for(&self, order: usize) -> f64 {
        let mut phi = 0.0;

        for (elt, cnt) in self.composition.iter() {
            let element = elt.element;
            phi += self
                .constants
                .nth_element_power_sum(element.symbol.as_ref(), order)
                * (*cnt as f64);
        }
        return phi;
    }

    pub fn phi_mass_for(&self, element: &'lifespan ElementSpecification, order: usize) -> f64 {
        let mut phi = self.composition.iter().fold(0.0, |phi, (elt, cnt)| {
            let coef = if elt.element == element.element {
                cnt - 1
            } else {
                *cnt
            };
            phi + self
                .constants
                .nth_element_power_sum(elt.element.symbol.as_ref(), order)
                * coef as f64
        });
        phi += self
            .constants
            .nth_element_power_sum_mass(element.element.symbol.as_ref(), order);
        return phi;
    }

    pub fn phi_values(&self, accumulator: &mut DVec) {
        accumulator.push(0.0);
        (1..(self.order as usize + 1)).for_each(|i| {
            accumulator.push(self.phi_for(i));
        });
    }

    pub fn phi_values_mass(
        &self,
        element: &'lifespan ElementSpecification,
        accumulator: &mut DVec,
    ) {
        accumulator.push(0.0);
        (1..(self.order as usize + 1)).for_each(|i| {
            accumulator.push(self.phi_mass_for(element, i));
        });
    }

    pub fn probability_vector(&self) -> DVec {
        let mut phi_vector = DVec::with_capacity(self.order as usize + 2);
        self.phi_values(&mut phi_vector);
        let n = phi_vector.len();

        // The probability vector will be in the elementary symmetric polynomoial
        let mut params = PolynomialParameters {
            power_sum: phi_vector,
            elementary_symmetric_polynomial: DVec::with_capacity(n),
        };
        params.newton_optimization(self.max_variants);

        for i in 0..params.elementary_symmetric_polynomial.len() {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            params.elementary_symmetric_polynomial[i] *= self.monoisotopic_peak.intensity * sign;
        }
        return params.elementary_symmetric_polynomial;
    }

    fn build_polynomial_map(&self) -> ElementPolynomialMap {
        let mut power_sum = DVec::new();
        let mut ep_map = ElementPolynomialMap::new(self.composition.len());

        for (elt, _) in self.composition.iter() {
            power_sum.clear();
            self.phi_values_mass(elt, &mut power_sum);
            let elementary_symmetric_polynomial = DVec::with_capacity(power_sum.len());
            let mut param = PolynomialParameters {
                elementary_symmetric_polynomial,
                power_sum,
            };
            param.newton_optimization(self.max_variants);
            power_sum = param.power_sum;
            ep_map.set(
                elt.element.symbol.as_ref(),
                param.elementary_symmetric_polynomial,
            );
        }
        return ep_map;
    }

    pub fn center_mass_vector(&self, probability_vector: &DVec) -> DVec {
        let mut mass_vector = DVec::with_capacity(probability_vector.len() + 3);
        let base_intensity = self.monoisotopic_peak.intensity;

        let ep_map = self.build_polynomial_map();

        for i in 0..(self.order + 1) as usize {
            let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
            let mut center = 0.0;
            for (elt, cnt) in self.composition.iter() {
                let element = elt.element;
                let ele_sym_poly = ep_map.get(element.symbol.as_ref());
                let mono_mass = element.most_abundant_mass;
                let polynomial_term = ele_sym_poly[i];
                center += (*cnt as f64) * (sign * polynomial_term) * base_intensity * mono_mass;
            }
            if probability_vector[i] == 0.0 {
                mass_vector.push(0.0);
            } else {
                mass_vector.push(center / probability_vector[i]);
            }
        }
        return mass_vector;
    }

    pub fn isotopic_variants(&self, charge: i32, charge_carrier: f64) -> PeakList {
        let probability_vector = self.probability_vector();
        let center_mass_vector = self.center_mass_vector(&probability_vector);

        let total: f64 = probability_vector.iter().sum();
        let mut peak_list = PeakList::with_capacity((self.order + 1) as usize);

        (0..self.order as usize + 1).for_each(|i| {
            let center_mass_i = center_mass_vector[i];
            let intensity_i = probability_vector[i];

            let adjusted_mz = if charge != 0 {
                mass_charge_ratio(center_mass_i, charge, charge_carrier)
            } else {
                center_mass_i
            };

            let peak = Peak {
                mz: adjusted_mz,
                intensity: intensity_i / total,
                charge: charge,
            };

            if peak.intensity < 1e-10 {
                return
            }

            peak_list.push(peak);
        });

        peak_list.sort_by(|a, b| a.mz.partial_cmp(&b.mz).unwrap());
        return peak_list;
    }
}

/// Generate a coarse isotopic pattern from a [`ChemicalComposition`]
/// with the specified peak count and charge state.
///
/// if `npeaks` is 0, a guess will be used.
pub fn isotopic_variants<'a, C: Into<ChemicalComposition<'a>>>(
    composition: C,
    npeaks: i32,
    charge: i32,
    charge_carrier: f64,
) -> PeakList {
    let composition = composition.into();
    let npeaks = if npeaks == 0 {
        guess_npeaks(&composition, 300)
    } else {
        npeaks - 1
    };

    let dist = IsotopicDistribution::from_composition(composition, npeaks);
    dist.isotopic_variants(charge, charge_carrier)
}

#[derive(Debug, Clone)]
pub struct BafflingRecursiveIsotopicPatternGenerator<'lifespan> {
    parameter_cache: IsotopicConstantsCache<'lifespan>,
}

impl<'transient, 'lifespan: 'transient, 'outer: 'lifespan>
    BafflingRecursiveIsotopicPatternGenerator<'lifespan>
{
    pub fn new() -> BafflingRecursiveIsotopicPatternGenerator<'lifespan> {
        BafflingRecursiveIsotopicPatternGenerator {
            parameter_cache: IsotopicConstantsCache::new(),
        }
    }

    #[inline]
    pub fn isotopic_variants<C: Into<ChemicalComposition<'outer>>>(
        &mut self,
        composition: C,
        npeaks: i32,
        charge: i32,
        charge_carrier: f64,
    ) -> PeakList {
        let composition = composition.into();
        let npeaks = if npeaks == 0 {
            guess_npeaks(&composition, 300)
        } else {
            npeaks - 1
        };
        let mut dist = IsotopicDistribution::fill_from_composition(composition, npeaks);
        dist.populate_constants_from_cache(&mut self.parameter_cache);
        let peaks = dist.isotopic_variants(charge, charge_carrier);
        self.parameter_cache.receive_from(dist.constants);
        return peaks;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::poisson_approximate_n_peaks_of;
    use crate::PROTON;

    #[test]
    fn test_baffling() {
        let comp = ChemicalComposition::parse("C6H12O6").unwrap();
        let peaks = isotopic_variants(comp, 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6)
    }

    #[test]
    fn test_sulfur() {
        let comp = ChemicalComposition::parse("C6H13O5S1H3").unwrap();
        let peaks = isotopic_variants(comp, 0, 1, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].neutral_mass() - 200.071846).abs() < 1e-6);
        assert!((peaks[0].intensity() - 0.8782583).abs() < 1e-6);
    }

    #[test]
    fn test_baffling_generator() {
        let comp = ChemicalComposition::parse("C6H12O6").unwrap();
        let mut generator = BafflingRecursiveIsotopicPatternGenerator::new();
        let peaks = generator.isotopic_variants(comp.clone(), 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6);
        let peaks = generator.isotopic_variants(comp.clone(), 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6);
    }

    #[test]
    fn test_max_variants() {
        let comp = ChemicalComposition::parse("C6H12O6").unwrap();
        let comp = comp * 6;
        let m = comp.mass();
        let max_vars = guess_npeaks(&comp, 300);
        let approx = poisson_approximate_n_peaks_of(m, 0.999);
        eprintln!("{} > {}", max_vars, approx);
    }
}
