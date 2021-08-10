//! An implementation of the Baffling Recursive Algorithm for Isotopic distributioN (BRAIN)
//! originally published in [Dittwald, 2013](http://dx.doi.org/10.1021/ac303439m).
use std::cmp;
use std::collections::hash_map::Entry;
use std::collections::HashMap;

use crate::element::Element;
use crate::isotopic_pattern::{Peak, PeakList};
use crate::{mass_charge_ratio, ChemicalComposition, ElementSpecification};

pub type DVec = Vec<f64>;

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

        for k in begin..end {
            if k == 0 {
                self.elementary_symmetric_polynomial.push(1.0);
            } else if k > (order as usize) {
                self.elementary_symmetric_polynomial.push(0.0);
            } else {
                let mut el = 0.0;
                for j in 1..k + 1 {
                    let sign = if (j % 2) == 1 { 1.0 } else { -1.0 };
                    el += sign * self.power_sum[j] * self.elementary_symmetric_polynomial[k - j];
                }
                el /= k as f64;
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
            let k = (n - i - 1 + monoisotopic_number) as u16;
            let isotope = element.isotopes.get(&k).unwrap();
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

#[derive(Debug)]
pub struct PhiConstants<'a> {
    pub order: i32,
    pub element: &'a Element,
    pub element_coefficients: PolynomialParameters,
    pub mass_coefficients: PolynomialParameters,
}

impl<'a> PhiConstants<'a> {
    pub fn from_element(element: &Element) -> PhiConstants {
        let mut accumulator = DVec::new();
        let order = element.max_neutron_shift as i32;
        let element_coefficients =
            PolynomialParameters::from_element(element, false, &mut accumulator);
        accumulator.clear();
        let mass_coefficients = PolynomialParameters::from_element(element, true, &mut accumulator);
        return PhiConstants {
            element,
            order,
            element_coefficients,
            mass_coefficients,
        };
    }
}

#[derive(Debug)]
pub struct IsotopicConstants<'a> {
    pub constants: HashMap<&'a str, PhiConstants<'a>>,
    pub order: i32,
}

impl<'a> IsotopicConstants<'a> {
    pub fn new(size: usize) -> IsotopicConstants<'a> {
        IsotopicConstants {
            constants: HashMap::with_capacity(size),
            order: 0,
        }
    }

    pub fn get(&self, symbol: &str) -> Option<&PhiConstants> {
        self.constants.get(symbol)
    }

    pub fn set(&mut self, symbol: &'a str, constants: PhiConstants<'a>) {
        self.constants.insert(symbol, constants);
    }

    pub fn add(&mut self, element: &'a Element) {
        match self.get(element.symbol.as_ref()) {
            Some(_c) => return,
            None => {}
        };

        let phi = PhiConstants::from_element(element);
        self.constants.insert(element.symbol.as_ref(), phi);
    }

    pub fn update(&mut self) {
        for (_symbol, elt_params) in self.constants.iter_mut() {
            if self.order < elt_params.order {
                continue;
            }
            for _j in elt_params.order..self.order + 1 {
                elt_params
                    .element_coefficients
                    .elementary_symmetric_polynomial
                    .push(0.0);
                elt_params
                    .mass_coefficients
                    .elementary_symmetric_polynomial
                    .push(0.0);
            }

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

    pub fn nth_element_power_sum(&self, symbol: &str, order: usize) -> f64 {
        let phi = self
            .get(symbol)
            .expect(format!("Expected element {} in constants", symbol).as_ref());
        phi.element_coefficients.power_sum[order]
    }

    pub fn nth_element_power_sum_mass(&self, symbol: &str, order: usize) -> f64 {
        let phi = self
            .get(symbol)
            .expect(format!("Expected element {} in constants", symbol).as_ref());
        phi.mass_coefficients.power_sum[order]
    }
}

pub struct IsotopicConstantsCache<'a> {
    pub cache: HashMap<&'a str, PhiConstants<'a>>,
}

impl<'a> IsotopicConstantsCache<'a> {
    pub fn new() -> IsotopicConstantsCache<'a> {
        return IsotopicConstantsCache {
            cache: HashMap::with_capacity(6),
        };
    }

    pub fn checkout(&mut self, symbol: &'a str) -> Option<PhiConstants<'a>> {
        self.cache.remove(symbol)
    }

    pub fn receive(&mut self, symbol: &'a str, constants: PhiConstants<'a>) -> bool {
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

    pub fn receive_from(&mut self, params: &mut IsotopicConstants<'a>) {
        for (k, v) in params.constants.drain() {
            self.receive(k, v);
        }
    }
}

pub fn max_variants(composition: &ChemicalComposition) -> i32 {
    let mut acc = 0;
    for (elt, cnt) in composition.iter() {
        acc += elt.element.max_neutron_shift as i32 * cnt;
    }
    acc
}

pub fn guess_npeaks(composition: &ChemicalComposition, max_npeaks: i32) -> i32 {
    let total_variants = max_variants(composition);
    let npeaks = (total_variants as f64).sqrt() as i32 - 2;
    let result = cmp::min(cmp::max(npeaks, 3), max_npeaks);
    return result;
}

struct ElementPolynomialMap<'a> {
    pub polynomials: HashMap<&'a str, DVec>,
}

impl<'a> ElementPolynomialMap<'a> {
    pub fn new(size: usize) -> ElementPolynomialMap<'a> {
        ElementPolynomialMap {
            polynomials: HashMap::with_capacity(size),
        }
    }

    pub fn set(&mut self, symbol: &'a str, polynomial: DVec) {
        self.polynomials.insert(symbol, polynomial);
    }

    pub fn get(&self, symbol: &'a str) -> &DVec {
        &self.polynomials[symbol]
    }
}

#[derive(Debug)]
pub struct IsotopicDistribution<'a> {
    pub composition: &'a ChemicalComposition<'a>,
    pub constants: IsotopicConstants<'a>,
    pub order: i32,
    pub average_mass: f64,
    pub monoisotopic_peak: Peak,
    pub max_variants: i32,
}

impl<'a> IsotopicDistribution<'a> {
    pub fn from_composition(
        composition: &'a ChemicalComposition<'a>,
        order: i32,
    ) -> IsotopicDistribution {
        let mut inst = IsotopicDistribution::fill_from_composition(composition, order);
        inst.populate_constants();
        inst
    }

    fn fill_from_composition(
        composition: &'a ChemicalComposition<'a>,
        order: i32,
    ) -> IsotopicDistribution {
        let mut inst = IsotopicDistribution {
            composition,
            order: 0,
            average_mass: 0.0,
            constants: IsotopicConstants::new(composition.len()),
            monoisotopic_peak: Peak {
                mz: 0.0,
                intensity: 0.0,
                charge: 0,
            },
            max_variants: max_variants(composition),
        };
        inst.update_order(order);
        inst.monoisotopic_peak = inst.make_monoisotopic_peak();
        inst
    }

    pub fn from_composition_and_cache<'outer: 'inner, 'inner: 'transient, 'transient>(
        composition: &'a ChemicalComposition<'outer>,
        order: i32,
        cache: &'inner mut IsotopicConstantsCache<'a>,
    ) -> IsotopicDistribution<'a> {
        let mut inst = IsotopicDistribution::fill_from_composition(composition, order);
        inst.populate_constants_from_cache(cache);
        inst
    }

    fn populate_constants_from_cache<'transient>(
        &mut self,
        cache: &'transient mut IsotopicConstantsCache<'a>,
    ) {
        for (elt, _cnt) in self.composition.iter() {
            match cache.checkout(elt.element.symbol.as_ref()) {
                None => {
                    self.constants.add(elt.element);
                }
                Some(isoconst) => {
                    self.constants.set(elt.element.symbol.as_ref(), isoconst);
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

    pub fn phi_mass_for(&self, element: &'a ElementSpecification, order: usize) -> f64 {
        let mut phi = 0.0;
        for (elt, cnt) in self.composition.iter() {
            let coef = if elt.element == element.element {
                cnt - 1
            } else {
                *cnt
            };
            phi += self
                .constants
                .nth_element_power_sum(elt.element.symbol.as_ref(), order)
                * coef as f64;
        }
        phi += self
            .constants
            .nth_element_power_sum_mass(element.element.symbol.as_ref(), order);
        return phi;
    }

    pub fn phi_values(&self, accumulator: &mut DVec) {
        accumulator.push(0.0);
        for i in 1..(self.order as usize) + 1 {
            accumulator.push(self.phi_for(i));
        }
    }

    pub fn phi_values_mass(&self, element: &'a ElementSpecification, accumulator: &mut DVec) {
        accumulator.push(0.0);
        for i in 1..(self.order as usize) + 1 {
            accumulator.push(self.phi_mass_for(element, i));
        }
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

        for (elt, _cnt) in self.composition.iter() {
            power_sum.clear();
            self.phi_values_mass(elt, &mut power_sum);
            let ele_sym_poly = DVec::new();
            let mut param = PolynomialParameters {
                elementary_symmetric_polynomial: ele_sym_poly,
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

        for i in 0..self.order as usize + 1 {
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
                continue;
            }

            peak_list.push(peak);
        }

        peak_list.sort_by(|a, b| a.mz.partial_cmp(&b.mz).unwrap());
        // average_mass /= total;
        return peak_list;
    }
}


/// Generate a coarse isotopic pattern from a [`ChemicalComposition`]
/// with the specified peak count and charge state.
///
/// if `npeaks` is 0, a guess will be used.
pub fn isotopic_variants<'a>(
    composition: &'a ChemicalComposition<'a>,
    npeaks: i32,
    charge: i32,
    charge_carrier: f64,
) -> PeakList {
    let npeaks = if npeaks == 0 {
        guess_npeaks(composition, 300)
    } else {
        npeaks - 1
    };

    let dist = IsotopicDistribution::from_composition(composition, npeaks);
    dist.isotopic_variants(charge, charge_carrier)
}

pub struct BafflingRecursiveIsotopicPatternGenerator<'a> {
    parameter_cache: IsotopicConstantsCache<'a>,
}

impl<'a> BafflingRecursiveIsotopicPatternGenerator<'a> {
    pub fn new() -> BafflingRecursiveIsotopicPatternGenerator<'a> {
        BafflingRecursiveIsotopicPatternGenerator {
            parameter_cache: IsotopicConstantsCache::new(),
        }
    }

    pub fn isotopic_variants<'outer: 'a>(
        &mut self,
        composition: &'outer ChemicalComposition<'outer>,
        npeaks: i32,
        charge: i32,
        charge_carrier: f64,
    ) -> PeakList {
        let npeaks = if npeaks == 0 {
            guess_npeaks(composition, 300)
        } else {
            npeaks - 1
        };
        let mut dist = IsotopicDistribution::from_composition_and_cache(
            composition,
            npeaks,
            &mut self.parameter_cache,
        );
        let peaks = dist.isotopic_variants(charge, charge_carrier);
        self.parameter_cache.receive_from(&mut dist.constants);
        return peaks;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::PROTON;

    #[test]
    fn test_baffling() {
        let comp = ChemicalComposition::parse("C6H12O6").unwrap();
        let peaks = isotopic_variants(&comp, 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6)
    }

    #[test]
    fn test_baffling_generator() {
        let comp = ChemicalComposition::parse("C6H12O6").unwrap();
        let mut generator = BafflingRecursiveIsotopicPatternGenerator::new();
        let peaks = generator.isotopic_variants(&comp, 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6);
        let peaks = generator.isotopic_variants(&comp, 5, 0, PROTON);
        assert_eq!(peaks.len(), 5);
        assert!((peaks[0].mz - 180.06339).abs() < 1e-6);
        assert!((peaks[0].intensity - 0.9226372340115745).abs() < 1e-6);
    }
}
