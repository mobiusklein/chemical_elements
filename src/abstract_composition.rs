#![allow(unused)]
use std::collections::hash_map::{Iter as HashMapIter, IterMut as HashMapIterMut};
use std::iter::{FromIterator, FusedIterator};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter as VecIter, IterMut as VecIterMut};
use std::str::FromStr;

#[cfg(feature="serde")]
use serde_with::SerializeDisplay;

use crate::formula::FormulaParser;
use crate::{
    ChemicalCompositionLike, ChemicalCompositionMap, ChemicalCompositionVec, ElementSpecification,
    FormulaParserError, PeriodicTable, PERIODIC_TABLE,
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(SerializeDisplay))]
pub enum ChemicalComposition<'lifespan> {
    Vec(ChemicalCompositionVec<'lifespan>),
    Map(ChemicalCompositionMap<'lifespan>),
}

impl<'lifespan> PartialEq for ChemicalComposition<'lifespan> {
    #[inline]
    fn eq(&self, other: &ChemicalComposition<'lifespan>) -> bool {
        if self.len() != other.len() {
            false
        } else {
            self.iter().all(|(k, v)| other.get(k) == *v)
        }
    }
}

impl<'lifespan> Default for ChemicalComposition<'lifespan> {
    fn default() -> Self {
        ChemicalComposition::Vec(ChemicalCompositionVec::default())
    }
}

impl<'transient, 'inner: 'transient, 'lifespan: 'inner> ChemicalComposition<'lifespan> {
    /// Create a new, empty [`ChemicalComposition`]
    pub fn new() -> ChemicalComposition<'lifespan> {
        Self::default()
    }

    #[inline]
    /// Access a specific element's count, or `0` if that element is absent
    /// from the composition
    pub fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        match self {
            ChemicalComposition::Vec(v) => v.get(elt_spec),
            ChemicalComposition::Map(m) => m.get(elt_spec),
        }
    }

    pub fn get_str(&self, sym: &str) -> i32 {
        match self {
            ChemicalComposition::Vec(v) => *v.index(sym),
            ChemicalComposition::Map(m) => m.get_str(sym),
        }
    }

    #[inline]
    /// Set the count for a specific element. This will invalidate the mass cache.
    pub fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        match self {
            ChemicalComposition::Vec(v) => v.set(elt_spec, count),
            ChemicalComposition::Map(m) => m.set(elt_spec, count),
        }
    }

    #[inline]
    /// Add some value to the count of the specified element. This will invalidate the
    /// mass cache.
    pub fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        match self {
            ChemicalComposition::Vec(v) => v.inc(elt_spec, count),
            ChemicalComposition::Map(m) => m.inc(elt_spec, count),
        }
    }

    #[inline]
    /// Add some value to the count of the specified element. This will invalidate the
    /// mass cache.
    pub fn inc_str(&mut self, elt_spec: &str, count: i32) {
        match self {
            ChemicalComposition::Vec(v) => *v.index_mut(elt_spec) += count,
            ChemicalComposition::Map(m) => m.inc_str(elt_spec, count),
        }
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
        match self {
            ChemicalComposition::Vec(v) => v.calc_mass(),
            ChemicalComposition::Map(m) => m.calc_mass(),
        }
    }

    #[inline]
    /**
    Get the mass of this chemical composition. If the mass cache
    has been populated, return that instead of repeating the calculation.
    */
    pub fn mass(&self) -> f64 {
        match self {
            ChemicalComposition::Vec(v) => v.mass(),
            ChemicalComposition::Map(m) => m.mass(),
        }
    }

    #[inline]
    /**
    Get the mass of this chemical composition, and cache it,
    or reuse the cached value. This requires mutability, so this method
    must be called explicitly.
    */
    pub fn fmass(&mut self) -> f64 {
        match self {
            ChemicalComposition::Vec(v) => v.fmass(),
            ChemicalComposition::Map(m) => m.fmass(),
        }
    }

    #[inline]
    /// Test if the mass cache is populated.
    pub fn has_mass_cached(&self) -> bool {
        match self {
            ChemicalComposition::Vec(v) => v.has_mass_cached(),
            ChemicalComposition::Map(m) => m.has_mass_cached(),
        }
    }

    #[inline]
    pub(crate) fn _add_from(&mut self, other: &'transient ChemicalCompositionVec<'lifespan>) {
        for (key, val) in other.iter() {
            self.inc(key.clone(), *val);
        }
    }

    #[inline]
    pub(crate) fn _sub_from(&mut self, other: &'transient ChemicalCompositionVec<'lifespan>) {
        for (key, val) in other.iter() {
            self.inc(key.clone(), -(*val));
        }
    }

    #[inline]
    pub(crate) fn _mul_by(&mut self, scaler: i32) {
        match self {
            ChemicalComposition::Vec(c) => c._mul_by(scaler),
            ChemicalComposition::Map(c) => c._mul_by(scaler),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ChemicalComposition::Vec(i) => i.is_empty(),
            ChemicalComposition::Map(i) => i.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ChemicalComposition::Vec(i) => i.len(),
            ChemicalComposition::Map(i) => i.len(),
        }
    }

    pub fn iter(&'inner self) -> Iter<'transient, 'lifespan> {
        match self {
            ChemicalComposition::Vec(inner) => Iter::Vec(inner.iter()),
            ChemicalComposition::Map(inner) => Iter::Map(inner.iter()),
        }
    }

    pub fn iter_mut(&'inner mut self) -> IterMut<'transient, 'lifespan> {
        match self {
            ChemicalComposition::Vec(chemical_composition_vec) => IterMut::Vec(chemical_composition_vec.iter_mut()),
            ChemicalComposition::Map(chemical_composition_map) => IterMut::Map(chemical_composition_map.iter_mut()),
        }
    }

    pub fn into_map(self) -> Self {
        match self {
            ChemicalComposition::Vec(c) => Self::Map(c.into()),
            ChemicalComposition::Map(c) => Self::Map(c),
        }
    }

    pub fn into_vec(self) -> Self {
        match self {
            ChemicalComposition::Vec(c) => Self::Vec(c),
            ChemicalComposition::Map(c) => Self::Vec(c.into()),
        }
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
    let hexose: ChemicalComposition = "C6O6(H2)6".parse().unwrap();
    assert_eq!(hexose["C"], 6);
    assert_eq!(hexose["O"], 6);
    assert_eq!(hexose["H"], 12);
    ```
    */
    pub fn parse(string: &str) -> Result<Self, FormulaParserError> {
        string.parse()
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
        let mut parser = FormulaParser::default();
        parser.parse_formula_with_table_generic(string, periodic_table)
    }
}

impl<'a> Index<&ElementSpecification<'a>> for ChemicalComposition<'a> {
    type Output = i32;

    #[inline]
    fn index(&self, key: &ElementSpecification<'a>) -> &Self::Output {
        match self {
            ChemicalComposition::Vec(c) => c.index(key),
            ChemicalComposition::Map(c) => c.index(key),
        }
    }
}

impl<'a> IndexMut<&ElementSpecification<'a>> for ChemicalComposition<'a> {
    #[inline]
    fn index_mut(&mut self, key: &ElementSpecification<'a>) -> &mut Self::Output {
        match self {
            ChemicalComposition::Vec(c) => c.index_mut(key),
            ChemicalComposition::Map(c) => c.index_mut(key),
        }
    }
}

impl<'a> Index<&str> for ChemicalComposition<'a> {
    type Output = i32;

    #[inline]
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            ChemicalComposition::Vec(c) => c.index(key),
            ChemicalComposition::Map(c) => c.index(key),
        }
    }
}

impl<'a> IndexMut<&str> for ChemicalComposition<'a> {
    #[inline]
    fn index_mut(&mut self, key: &str) -> &mut Self::Output {
        match self {
            ChemicalComposition::Vec(c) => c.index_mut(key),
            ChemicalComposition::Map(c) => c.index_mut(key),
        }
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

impl<'transient, 'lifespan: 'transient>
    FromIterator<(&'lifespan ElementSpecification<'lifespan>, &'transient i32)>
    for ChemicalComposition<'lifespan>
{
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (&'lifespan ElementSpecification<'lifespan>, &'transient i32)>,
    {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            let elt_spec = k.clone();
            composition.inc(elt_spec, *v);
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
        let mut composition = ChemicalComposition::new();
        elements.iter().cloned().for_each(|(k, v)| {
            composition.inc(k, v);
        });
        return composition;
    }
}

#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub enum Iter<'inner, 'lifespan: 'inner> {
    Vec(std::slice::Iter<'inner, (ElementSpecification<'lifespan>, i32)>),
    Map(std::collections::hash_map::Iter<'inner, ElementSpecification<'lifespan>, i32>)
}

impl<'inner, 'lifespan: 'inner> FusedIterator for Iter<'inner, 'lifespan> {}

impl<'inner, 'lifespan: 'inner> ExactSizeIterator for Iter<'inner, 'lifespan> {
    fn len(&self) -> usize {
        match self {
            Iter::Vec(iter) => iter.len(),
            Iter::Map(iter) => iter.len(),
        }
    }
}

impl<'inner, 'lifespan: 'inner> Iterator for Iter<'inner, 'lifespan> {
    type Item = (&'inner ElementSpecification<'lifespan>, &'inner i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'inner, 'lifespan: 'inner> Iter<'inner, 'lifespan> {
    fn next(&mut self) -> Option<(&'inner ElementSpecification<'lifespan>, &'inner i32)> {
        match self {
            Iter::Vec(iter) => {
                iter.next().map(|(k, v)| (k, v))
            },
            Iter::Map(iter) => {
                iter.next()
            },
        }
    }
}


#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub enum IterMut<'inner, 'lifespan: 'inner> {
    Vec(std::slice::IterMut<'inner, (ElementSpecification<'lifespan>, i32)>),
    Map(std::collections::hash_map::IterMut<'inner, ElementSpecification<'lifespan>, i32>)
}

impl<'inner, 'lifespan: 'inner> FusedIterator for IterMut<'inner, 'lifespan> {}

impl<'inner, 'lifespan: 'inner> ExactSizeIterator for IterMut<'inner, 'lifespan> {
    fn len(&self) -> usize {
        match self {
            IterMut::Vec(iter_mut) => iter_mut.len(),
            IterMut::Map(iter_mut) => iter_mut.len(),
        }
    }
}

impl<'inner, 'lifespan: 'inner> Iterator for IterMut<'inner, 'lifespan> {
    type Item = (&'inner ElementSpecification<'inner>, &'inner mut i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl<'inner, 'lifespan: 'inner> IterMut<'inner, 'lifespan> {
    fn next(&mut self) -> Option<(&'inner ElementSpecification<'inner>, &'inner mut i32)> {
        match self {
            IterMut::Vec(iter_mut) => {
                iter_mut.next().map(|(k, v)| (&*k, v))
            },
            IterMut::Map(iter_mut) => iter_mut.next(),
        }
    }
}


impl<'a> FromStr for ChemicalComposition<'a> {
    type Err = FormulaParserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = FormulaParser::default();
        parser.parse_formula_with_table_generic(s, &PERIODIC_TABLE)
    }
}

impl<'lifespan> ToString for ChemicalComposition<'lifespan> {
    fn to_string(&self) -> String {
        crate::formula::to_formula(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChemicalCompositionRef<'inner, 'lifespan: 'inner> {
    Vec(&'inner ChemicalCompositionVec<'lifespan>),
    Map(&'inner ChemicalCompositionMap<'lifespan>),
}

impl<'transient, 'inner: 'transient, 'lifespan: 'inner> ChemicalCompositionRef<'inner, 'lifespan> {
    #[inline]
    /// Access a specific element's count, or `0` if that element is absent
    /// from the composition
    pub fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        match self {
            ChemicalCompositionRef::Vec(v) => v.get(elt_spec),
            ChemicalCompositionRef::Map(m) => m.get(elt_spec),
        }
    }

    #[inline]
    /**
    Explicitly calculate the mass of the chemical composition, ignoring
    any caching.
    */
    pub fn calc_mass(&self) -> f64 {
        match self {
            ChemicalCompositionRef::Vec(v) => v.calc_mass(),
            ChemicalCompositionRef::Map(m) => m.calc_mass(),
        }
    }

    #[inline]
    /**
    Get the mass of this chemical composition. If the mass cache
    has been populated, return that instead of repeating the calculation.
    */
    pub fn mass(&self) -> f64 {
        match self {
            ChemicalCompositionRef::Vec(v) => v.mass(),
            ChemicalCompositionRef::Map(m) => m.mass(),
        }
    }

    #[inline]
    /// Test if the mass cache is populated.
    pub fn has_mass_cached(&self) -> bool {
        match self {
            ChemicalCompositionRef::Vec(v) => v.has_mass_cached(),
            ChemicalCompositionRef::Map(m) => m.has_mass_cached(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ChemicalCompositionRef::Vec(i) => i.is_empty(),
            ChemicalCompositionRef::Map(i) => i.is_empty(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            ChemicalCompositionRef::Vec(i) => i.len(),
            ChemicalCompositionRef::Map(i) => i.len(),
        }
    }

    pub fn iter(&'inner self) -> Iter<'inner, 'lifespan> {
        match self {
            ChemicalCompositionRef::Vec(chemical_composition_vec) => Iter::Vec(chemical_composition_vec.iter()),
            ChemicalCompositionRef::Map(chemical_composition_map) => Iter::Map(chemical_composition_map.iter()),
        }
    }
}

impl<'a, 'b: 'a> Index<&ElementSpecification<'b>> for ChemicalCompositionRef<'a, 'b> {
    type Output = i32;

    fn index(&self, index: &ElementSpecification<'b>) -> &Self::Output {
        match self {
            ChemicalCompositionRef::Vec(c) => c.index(index),
            ChemicalCompositionRef::Map(c) => c.index(index),
        }
    }
}

impl<'a, 'b: 'a> Index<&str> for ChemicalCompositionRef<'a, 'b> {
    type Output = i32;

    #[inline]
    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Self::Vec(c) => c.index(key),
            Self::Map(c) => c.index(key),
        }
    }
}

impl<'inner, 'lifespan: 'inner> From<&'inner ChemicalComposition<'lifespan>>
    for ChemicalCompositionRef<'inner, 'lifespan>
{
    fn from(value: &'inner ChemicalComposition<'lifespan>) -> Self {
        match value {
            ChemicalComposition::Vec(v) => Self::Vec(v),
            ChemicalComposition::Map(m) => Self::Map(m),
        }
    }
}

impl<'inner, 'lifespan: 'inner> From<&'inner ChemicalCompositionVec<'lifespan>>
    for ChemicalCompositionRef<'inner, 'lifespan>
{
    fn from(value: &'inner ChemicalCompositionVec<'lifespan>) -> Self {
        ChemicalCompositionRef::Vec(value)
    }
}

impl<'inner, 'lifespan: 'inner> From<&'inner ChemicalCompositionMap<'lifespan>>
    for ChemicalCompositionRef<'inner, 'lifespan>
{
    fn from(value: &'inner ChemicalCompositionMap<'lifespan>) -> Self {
        ChemicalCompositionRef::Map(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let case: ChemicalComposition = "H2O".parse().expect("Failed to parse");
        eprintln!("{}", case.to_string());
        let mut ctrl = ChemicalComposition::new();
        ctrl.set(("O").parse::<ElementSpecification>().unwrap(), 1);
        ctrl.set(("H").parse::<ElementSpecification>().unwrap(), 2);
        eprintln!("{}", ctrl.to_string());
        assert_eq!(case, ctrl);
        let case: ChemicalComposition = "H2O1".parse().expect("Failed to parse");
        assert_eq!(case, ctrl);
        let case: ChemicalComposition = "(H)2O1".parse().expect("Failed to parse");
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
