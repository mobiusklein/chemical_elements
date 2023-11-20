#![allow(unused)]
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter, IterMut};

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

use crate::element_specification::{ElementSpecification, ElementSpecificationLike};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/**
Represents a collection of element-count pairs as found in a flat
chemical formula. Built atop [`std::collections::HashMap`], and
support addition and subtraction with other instances of the same type
and multiplication by integers.
*/
pub struct ChemicalComposition<'a> {
    pub composition: Vec<(ElementSpecification<'a>, i32)>,
    mass_cache: Option<f64>,
}

/**
# Basic Operations
*/
impl<'transient, 'lifespan: 'transient> ChemicalComposition<'lifespan> {
    /// Create a new, empty [`ChemicalComposition`]
    pub fn new() -> ChemicalComposition<'lifespan> {
        ChemicalComposition {
            ..Default::default()
        }
    }

    fn find(&self, elt_spec: &ElementSpecification<'lifespan>) -> Option<usize> {
        let found = self
            .composition
            .iter()
            .enumerate()
            .find(|(_, (e, _))| elt_spec == e);
        if let Some((index, _)) = found {
            Some(index)
        } else {
            None
        }
    }

    fn get_str(&self, elt_str: &str) -> &i32 {
        if let Some((_, c)) = self.composition.iter().find(|(e, _)| e == elt_str) {
            c
        } else {
            &ZERO
        }
    }

    #[inline]
    /// Access a specific element's count, or `0` if that element is absent
    /// from the composition
    pub fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        if let Some((_, c)) = self.composition.iter().find(|(e, _)| elt_spec == e) {
            *c
        } else {
            0
        }
    }

    #[inline]
    /// Set the count for a specific element. This will invalidate the mass cache.
    pub fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        if let Some(i) = self.find(&elt_spec) {
            self.composition[i].1 = count
        } else {
            self.composition.push((elt_spec, count));
        }
        self.mass_cache = None;
    }

    #[inline]
    /// Add some value to the count of the specified element. This will invalidate the
    /// mass cache.
    pub fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        let i = self.get(&elt_spec);
        self.set(elt_spec, i + count);
    }

    #[inline]
    pub fn iter(&self) -> Iter<(ElementSpecification<'lifespan>, i32)> {
        return (self.composition).iter();
    }

    pub fn iter_mut(&mut self) -> IterMut<(ElementSpecification<'lifespan>, i32)> {
        self.composition.iter_mut()
    }

    pub(crate) fn get_ref(&self) -> &[(ElementSpecification<'lifespan>, i32)] {
        &self.composition
    }

    #[allow(unused)]
    pub(crate) fn get_mut(&mut self) -> &mut [(ElementSpecification<'lifespan>, i32)] {
        &mut self.composition
    }

    /**
    Return [`self.composition`], consuming the object
    */
    pub fn into_inner(self) -> Vec<(ElementSpecification<'lifespan>, i32)> {
        self.composition
    }

    /**
    # Mass calculation Methods

    [`ChemicalComposition`] has three methods for computing the monoisotopic
    mass of the composition it represents to handle mutability.
    */

    #[inline]
    /**
    Explicitly calculate the mass of the chemical composition, ignoring
    any caching.
    */
    pub fn calc_mass(&self) -> f64 {
        let mut total = 0.0;
        for (elt_spec, count) in &self.composition {
            let element = elt_spec.element;
            total += if elt_spec.isotope == 0 {
                element.most_abundant_mass
            } else {
                element.isotopes[&elt_spec.isotope].mass
            } * (*count as f64);
        }
        return total;
    }

    #[inline]
    /**
    Get the mass of this chemical composition. If the mass cache
    has been populated, return that instead of repeating the calculation.
    */
    pub fn mass(&self) -> f64 {
        let mass = match self.mass_cache {
            None => self.calc_mass(),
            Some(val) => val,
        };
        return mass;
    }

    #[inline]
    /**
    Get the mass of this chemical composition, and cache it,
    or reuse the cached value. This requires mutability, so this method
    must be called explicitly.
    */
    pub fn fmass(&mut self) -> f64 {
        let mass = match self.mass_cache {
            None => {
                let total = self.mass();
                self.mass_cache = Some(total);
                total
            }
            Some(val) => val,
        };
        return mass;
    }

    #[inline]
    /// Test if the mass cache is populated.
    pub fn has_mass_cached(&self) -> bool {
        self.mass_cache.is_some()
    }
}

const ZERO: i32 = 0;

impl<'lifespan> Index<&ElementSpecification<'lifespan>> for ChemicalComposition<'lifespan> {
    type Output = i32;

    #[inline]
    fn index(&self, key: &ElementSpecification<'lifespan>) -> &Self::Output {
        if let Some(i) = self.find(key) {
            let (_, out) = self.composition.get(i).unwrap();
            out
        } else {
            &ZERO
        }
    }
}

impl<'lifespan> IndexMut<&ElementSpecification<'lifespan>> for ChemicalComposition<'lifespan> {
    #[inline]
    fn index_mut(&mut self, key: &ElementSpecification<'lifespan>) -> &mut Self::Output {
        self.mass_cache = None;
        if let Some(i) = self.find(key) {
            let (_, out) = self.composition.get_mut(i).unwrap();
            out
        } else {
            self.set(key.clone(), 0);
            let i = self.composition.len() - 1;
            let (_, out) = self.composition.get_mut(i).unwrap();
            out
        }
    }
}

impl<'lifespan> Index<&str> for ChemicalComposition<'lifespan> {
    type Output = i32;

    /**
    Using the [`Index`] trait to access element counts with a [`&str`] is more
    flexible than [`ChemicalComposition::get_str`], supporting fixed
    isotope strings, but does slightly more string checking up-front.
    */
    #[inline]
    fn index(&self, key: &str) -> &Self::Output {
        match ElementSpecification::quick_check_str(key) {
            ElementSpecificationLike::Yes => self.get_str(key),
            ElementSpecificationLike::No => &ZERO,
            ElementSpecificationLike::Maybe => {
                let spec = key.parse::<ElementSpecification>();
                match spec {
                    Ok(spec) => self.index(&spec),
                    Err(_err) => &ZERO,
                }
            }
        }
    }
}

impl<'lifespan> IndexMut<&str> for ChemicalComposition<'lifespan> {
    /** Using [`IndexMut`] with a [`&str`] will always construct a new
    [`ElementSpecification`] from the provided `&str`, in order to
    maintain the contract with with [`std::ops::Index`]
    */
    #[inline]
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        self.mass_cache = None;
        let key = key.parse::<ElementSpecification>().unwrap();
        let entry = self.index_mut(&key);
        entry
    }
}

impl<'lifespan, 'transient, 'outer: 'transient> ChemicalComposition<'lifespan> {
    #[inline]
    fn _add_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.iter() {
            self.inc(key.clone(), *val);
        }
    }

    #[inline]
    fn _sub_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.iter() {
            self.inc(key.clone(), -(*val));
        }
    }

    #[inline]
    fn _mul_by(&mut self, scaler: i32) {
        self.composition.iter_mut().for_each(|(_, v)| {
            *v *= scaler;
        })
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.composition.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.composition.is_empty()
    }
}

impl<'lifespan> PartialEq<ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
    #[inline]
    fn eq(&self, other: &ChemicalComposition<'lifespan>) -> bool {
        self.composition == other.composition
    }
}

// impl<'lifespan> Add<&ChemicalComposition<'lifespan>> for &ChemicalComposition<'lifespan> {
//     type Output = ChemicalComposition<'lifespan>;

//     #[inline]
//     fn add(self, other: &ChemicalComposition<'lifespan>) -> Self::Output {
//         let mut inst = self.clone();
//         inst._add_from(other);
//         return inst;
//     }
// }

// impl<'lifespan> Sub<&'lifespan ChemicalComposition<'_>> for &ChemicalComposition<'lifespan> {
//     type Output = ChemicalComposition<'lifespan>;

//     #[inline]
//     fn sub(self, other: &'lifespan ChemicalComposition<'_>) -> Self::Output {
//         let mut inst = self.clone();
//         inst._sub_from(other);
//         return inst;
//     }
// }

impl<'lifespan> Mul<i32> for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    #[inline]
    fn mul(self, other: i32) -> Self::Output {
        let mut inst = self.clone();
        inst._mul_by(other);
        return inst;
    }
}

// impl<'lifespan> AddAssign<&'_ ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
//     #[inline]
//     fn add_assign(&mut self, other: &ChemicalComposition<'lifespan>) {
//         self._add_from(other);
//     }
// }

// impl<'lifespan> AddAssign<ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
//     #[inline]
//     fn add_assign(&mut self, other: ChemicalComposition<'lifespan>) {
//         self._add_from(&other);
//     }
// }

// impl<'lifespan> SubAssign<&'_ ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
//     #[inline]
//     fn sub_assign(&mut self, other: &'_ ChemicalComposition<'lifespan>) {
//         self._sub_from(other);
//     }
// }

// impl<'lifespan> SubAssign<ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
//     #[inline]
//     fn sub_assign(&mut self, other: ChemicalComposition<'lifespan>) {
//         self._sub_from(&other);
//     }
// }

impl<'lifespan> MulAssign<i32> for ChemicalComposition<'_> {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        self._mul_by(other);
    }
}

impl<'lifespan> Neg for ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    #[inline]
    fn neg(mut self) -> Self::Output {
        self._mul_by(-1);
        self
    }
}

impl<'lifespan> Neg for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    #[inline]
    fn neg(self) -> Self::Output {
        let mut dup = self.clone();
        dup._mul_by(-1);
        dup
    }
}

impl<'lifespan> FromIterator<(ElementSpecification<'lifespan>, i32)>
    for ChemicalComposition<'lifespan>
{
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (ElementSpecification<'lifespan>, i32)>,
    {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            composition.inc(k, v);
        }
        return composition;
    }
}

impl<'lifespan> FromIterator<(&'lifespan str, i32)> for ChemicalComposition<'lifespan> {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'lifespan str, i32)>,
    {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            let elt_spec = ElementSpecification::parse(k).unwrap();
            composition.inc(elt_spec, v);
        }
        return composition;
    }
}

impl<'lifespan> From<Vec<(&'lifespan str, i32)>> for ChemicalComposition<'lifespan> {
    #[inline]
    fn from(elements: Vec<(&'lifespan str, i32)>) -> Self {
        let composition: ChemicalComposition<'lifespan> = elements.iter().cloned().collect();
        return composition;
    }
}

impl<'lifespan> From<Vec<(ElementSpecification<'lifespan>, i32)>>
    for ChemicalComposition<'lifespan>
{
    fn from(elements: Vec<(ElementSpecification<'lifespan>, i32)>) -> Self {
        let composition = ChemicalComposition {
            composition: elements,
            mass_cache: None,
        };
        return composition;
    }
}


#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_parse() {
    //     let case = ChemicalComposition::parse("H2O").expect("Failed to parse");
    //     let mut ctrl = ChemicalComposition::new();
    //     ctrl.set(("O").parse::<ElementSpecification>().unwrap(), 1);
    //     ctrl.set(("H").parse::<ElementSpecification>().unwrap(), 2);
    //     assert_eq!(case, ctrl);
    //     let case = ChemicalComposition::parse("H2O1").expect("Failed to parse");
    //     assert_eq!(case, ctrl);
    //     let case = ChemicalComposition::parse("(H)2O1").expect("Failed to parse");
    //     assert_eq!(case, ctrl);
    // }

    #[test]
    fn test_from_vec_str() {
        let case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
        let mut ctrl = ChemicalComposition::new();
        ctrl.set(("O").parse::<ElementSpecification>().unwrap(), 1);
        ctrl.set(("H").parse::<ElementSpecification>().unwrap(), 2);
        assert_eq!(case, ctrl);
    }

    #[test]
    fn test_from_vec_elt_spec() {
        let hydrogen = ("H").parse::<ElementSpecification>().unwrap();
        let oxygen = ("O").parse::<ElementSpecification>().unwrap();
        let case = ChemicalComposition::from(vec![(oxygen, 1), (hydrogen, 2)]);
        let mut ctrl = ChemicalComposition::new();

        let hydrogen = ("H").parse::<ElementSpecification>().unwrap();
        let oxygen = ("O").parse::<ElementSpecification>().unwrap();
        ctrl.set(oxygen, 1);
        ctrl.set(hydrogen, 2);
        assert_eq!(case, ctrl);
    }

    #[test]
    fn test_mass() {
        let case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
        let mass = 18.0105646837;

        let calc = case.mass();
        assert!((mass - calc).abs() < 1e-6);
    }

    #[test]
    fn test_fmass() {
        let mut case: ChemicalComposition = (vec![("O", 1), ("H", 2)]).into();
        let mass = 18.0105646837;

        let calc = case.fmass();
        assert!((mass - calc).abs() < 1e-6);
    }

    #[test]
    fn test_add() {
        let case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
        let ctrl = ChemicalComposition::from(vec![("O", 2), ("H", 4)]);

        let combo = &case + &case;
        assert_eq!(ctrl, combo);
    }

    #[test]
    fn test_sub() {
        let case = ChemicalComposition::from(vec![("O", 2), ("H", 4)]);
        let ctrl = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);

        let combo = &case - &ctrl;
        assert_eq!(ctrl, combo);
    }

    #[test]
    fn test_mul() {
        let case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
        let ctrl = ChemicalComposition::from(vec![("O", 2), ("H", 4)]);

        let combo = &case * 2;
        assert_eq!(ctrl, combo);
    }
}
