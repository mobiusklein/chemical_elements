#![allow(unused)]
use std::collections::hash_map::{HashMap, Iter, IterMut};
use std::iter::FromIterator;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::str::FromStr;

use ahash::RandomState;

#[cfg(feature = "serde1")]
use serde::{Deserialize, Serialize};

use crate::element::PeriodicTable;
use crate::element_specification::{ElementSpecification, ElementSpecificationLike};
use crate::formula::{parse_formula, parse_formula_with_table, FormulaParserError};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde1", derive(Serialize, Deserialize))]
/**
Represents a collection of element-count pairs as found in a flat
chemical formula. Built atop [`std::collections::HashMap`], and
support addition and subtraction with other instances of the same type
and multiplication by integers.
*/
pub struct ChemicalComposition<'a> {
    pub composition: HashMap<ElementSpecification<'a>, i32, RandomState>,
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

    #[inline]
    /// Access a specific element's count, or `0` if that element is absent
    /// from the composition
    pub fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        return match self.composition.get(elt_spec) {
            Some(i) => *i,
            None => 0,
        };
    }

    #[inline]
    /// Set the count for a specific element. This will invalidate the mass cache.
    pub fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.composition.insert(elt_spec, count);
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
    pub fn iter(&self) -> Iter<ElementSpecification<'lifespan>, i32> {
        return (self.composition).iter();
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<ElementSpecification<'lifespan>, i32> {
        return (self.composition).iter_mut();
    }

    /**
    Return [`self.composition`], consuming the object
    */
    pub fn into_inner(self) -> HashMap<ElementSpecification<'lifespan>, i32, RandomState> {
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

    /**
    # Formula String Parsing

    The formula notation supports fixed isotopes following elements enclosed in `[]`
    and parenthesized groups enclosed in `()`.

    Parse a text formula into a [`ChemicalComposition`] using the
    global [`PeriodicTable`].

    If the formula fails to parse, a [`FormulaParserError`] is returned.

    ```rust
    # use chemical_elements::ChemicalComposition;
    let hexose = ChemicalComposition::parse("C6O6(H2)6").unwrap();
    assert_eq!(hexose["C"], 6);
    assert_eq!(hexose["O"], 6);
    assert_eq!(hexose["H"], 12);
    ```
    */
    #[inline]
    pub fn parse(
        string: &'transient str,
    ) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        parse_formula(string)
    }

    #[inline]
    /**
    Parse a text formula into a [`ChemicalComposition`], using the specified
    [`PeriodicTable`], otherwise behaving identically to [`ChemicalComposition::parse`].
    */
    pub fn parse_with(
        string: &str,
        periodic_table: &'lifespan PeriodicTable,
    ) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        parse_formula_with_table(string, periodic_table)
    }
}

/**
*/
impl<'lifespan, 'transient, 'outer: 'transient> ChemicalComposition<'lifespan> {
    #[inline]
    fn _add_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.composition.iter() {
            self.inc(key.clone(), *val);
        }
    }

    #[inline]
    fn _sub_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.composition.iter() {
            self.inc(key.clone(), -(*val));
        }
    }

    #[inline]
    fn _mul_by(&mut self, scaler: i32) {
        self.iter_mut().for_each(|(k, v)| {
            *v *= scaler
        });
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

impl<'lifespan> Index<&ElementSpecification<'lifespan>> for ChemicalComposition<'lifespan> {
    type Output = i32;

    #[inline]
    fn index(&self, key: &ElementSpecification<'lifespan>) -> &Self::Output {
        let ent = self.composition.get(key);
        return ent.unwrap();
    }
}

impl<'lifespan> IndexMut<&ElementSpecification<'lifespan>> for ChemicalComposition<'lifespan> {
    #[inline]
    fn index_mut(&mut self, key: &ElementSpecification<'lifespan>) -> &mut Self::Output {
        self.mass_cache = None;
        let entry = self.composition.entry(key.clone());
        entry.or_insert(0)
    }
}

/**
# String-based accessors

When performing routine manipulations of a [`ChemicalComposition`] it may
be both more efficient and easier to write those operations using strings
or string literals, rather than instantiating an [`ElementSpecification`]
for each operation. These methods take advantage of the way
[`HashMap::get`](std::collections::HashMap::get) is parameterized to avoid
constructing a new [`ElementSpecification`] unless absolutely necessary.
*/
impl<'a> ChemicalComposition<'a> {
    /// Get the quantity of an element by its symbol string.
    ///
    /// This method does not support fixed isotopes, but may
    /// be faster as it skips element specification parsing and
    /// [`PeriodicTable`] lookup.
    pub fn get_str(&self, elt: &str) -> i32 {
        match self.composition.get(elt) {
            Some(c) => *c,
            None => 0,
        }
    }

    /**
    Get a mutable reference of quantity of an element by its symbol string,
    if it exists. This method invalidates the mass cache.

    This method does not support fixed isotopes, but may
    be faster as it skips element specification parsing and
    [`PeriodicTable`] lookup.

    # Note
    While the borrow checker should stop you from mutating the object
    while the borrowed count is still alive, unsafe use may allow the
    [`ChemicalComposition.mass_cache`] to get out of sync with updates
    to element counts.
    */
    pub fn get_str_mut(&mut self, elt: &str) -> Option<&mut i32> {
        self.mass_cache = None;
        self.composition.get_mut(elt)
    }

    /// Increment of quantity of an element by its symbol string,
    /// if it exists. This method invalidates the mass cache.
    ///
    /// This method does not support fixed isotopes, but may
    /// be faster as it skips element specification parsing and
    /// [`PeriodicTable`] lookup, if the element is already in
    /// the composition. Otherwise, the string is parsed and a new
    /// [`ElementSpecification`] is created using the default [`PeriodicTable`].
    ///
    /// # Panics
    /// If a new [`ElementSpecification`] needs to be created and fails,
    /// this method will panic.
    pub fn inc_str(&mut self, elt: &str, count: i32) {
        self.mass_cache = None;
        if let Some(val) = self.get_str_mut(elt) {
            *val += count;
        } else {
            match ElementSpecification::parse(elt) {
                Ok(spec) => self.inc(spec, count),
                Err(err) => {
                    panic!("Failed to parse element specification {} while incrementing composition: {:?}", elt, err)
                }
            }
        }
    }
}

const ZERO: i32 = 0;

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
            ElementSpecificationLike::Yes => self.composition.get(key).unwrap_or(&ZERO),
            ElementSpecificationLike::No => &ZERO,
            ElementSpecificationLike::Maybe => {
                let spec = key.parse::<ElementSpecification>();
                match spec {
                    Ok(spec) => self.composition.get(&spec).unwrap_or(&ZERO),
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
        let entry = self.composition.entry(key);
        entry.or_insert(0)
    }
}

impl<'lifespan> PartialEq<ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
    #[inline]
    fn eq(&self, other: &ChemicalComposition<'lifespan>) -> bool {
        self.composition == other.composition
    }
}

impl<'lifespan> Mul<i32> for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    #[inline]
    fn mul(self, other: i32) -> Self::Output {
        let mut inst = self.clone();
        inst._mul_by(other);
        return inst;
    }
}

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
        let composition: ChemicalComposition<'lifespan> = elements.iter().cloned().collect();
        return composition;
    }
}

impl<'a> FromStr for ChemicalComposition<'a> {
    type Err = FormulaParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ChemicalComposition::parse(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let case = ChemicalComposition::parse("H2O").expect("Failed to parse");
        let mut ctrl = ChemicalComposition::new();
        ctrl.set(("O").parse::<ElementSpecification>().unwrap(), 1);
        ctrl.set(("H").parse::<ElementSpecification>().unwrap(), 2);
        assert_eq!(case, ctrl);
        let case = ChemicalComposition::parse("H2O1").expect("Failed to parse");
        assert_eq!(case, ctrl);
        let case = ChemicalComposition::parse("(H)2O1").expect("Failed to parse");
        assert_eq!(case, ctrl);
    }

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
        let mut case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
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
