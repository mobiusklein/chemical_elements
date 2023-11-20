#![allow(unused)]
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use std::slice::{Iter as VecIter, IterMut as VecIterMut};
use std::iter::FromIterator;
use std::collections::hash_map::{Iter as HashMapIter, IterMut as HashMapIterMut};

use crate::{
    ChemicalComposition as ChemicalCompositionMap, ChemicalCompositionVec, ElementSpecification, ChemicalCompositionLike,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ChemicalComposition<'lifespan> {
    Vec(ChemicalCompositionVec<'lifespan>),
    Map(ChemicalCompositionMap<'lifespan>),
}

impl<'lifespan> Default for ChemicalComposition<'lifespan> {
    fn default() -> Self {
        ChemicalComposition::Vec(ChemicalCompositionVec::default())
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalComposition<'lifespan> {
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

    pub fn iter(&'transient self) -> Iter<'transient, 'lifespan> {
        Iter {
            composition: self,
            offest: 0
        }
    }

    pub fn iter_mut(&'transient mut self) -> IterMut<'transient, 'lifespan> {
        IterMut {
            composition: self,
            offest: 0
        }
    }
}

// impl<'a> Add for ChemicalComposition<'a> {
//     type Output = ChemicalComposition<'a>;

//     fn add(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//         }
//     }
// }

// impl<'a> Sub for ChemicalComposition<'a> {
//     type Output = ChemicalComposition<'a>;

//     fn sub(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//         }
//     }
// }

impl<'a> Mul<i32> for ChemicalComposition<'a> {
    type Output = ChemicalComposition<'a>;

    #[inline]
    fn mul(self, other: i32) -> Self::Output {
        match &self {
            ChemicalComposition::Vec(c) => ChemicalComposition::Vec(c * other),
            ChemicalComposition::Map(c) => ChemicalComposition::Map(c * other),
        }
    }
}

// impl<'a> Add for &ChemicalComposition<'a> {
//     type Output = ChemicalComposition<'a>;

//     fn add(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), *v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//         }
//     }
// }

// impl<'a> Sub for &ChemicalComposition<'a> {
//     type Output = ChemicalComposition<'a>;

//     fn sub(self, rhs: Self) -> Self::Output {
//         match (self, rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Vec(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 let mut dup = a.clone();
//                 b.iter().for_each(|(e, v)| {
//                     dup.inc(e.clone(), -*v);
//                 });
//                 ChemicalComposition::Map(dup)
//             }
//         }
//     }
// }

impl<'a> Mul<i32> for &ChemicalComposition<'a> {
    type Output = ChemicalComposition<'a>;

    #[inline]
    fn mul(self, other: i32) -> Self::Output {
        match &self {
            ChemicalComposition::Vec(c) => ChemicalComposition::Vec(c * other),
            ChemicalComposition::Map(c) => ChemicalComposition::Map(c * other),
        }
    }
}

// impl<'a> AddAssign<&ChemicalComposition<'a>> for ChemicalComposition<'a> {
//     fn add_assign(&mut self, rhs: &Self) {
//         match (self, &rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), *v);
//                 });
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), *v);
//                 });
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), *v);
//                 });
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), *v);
//                 });
//             }
//         }
//     }
// }

// impl<'a, 'b: 'a> SubAssign<&'a ChemicalComposition<'b>> for ChemicalComposition<'b> {
//     fn sub_assign(&mut self, rhs: &Self) {
//         match (self, rhs) {
//             (ChemicalComposition::Vec(a), ChemicalComposition::Vec(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), -*v);
//                 });
//             }
//             (ChemicalComposition::Vec(a), ChemicalComposition::Map(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), -*v);
//                 });
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Vec(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), -*v);
//                 });
//             }
//             (ChemicalComposition::Map(a), ChemicalComposition::Map(b)) => {
//                 b.iter().for_each(|(e, v)| {
//                     a.inc(e.clone(), -*v);
//                 });
//             }
//         }
//     }
// }

impl<'a> MulAssign<i32> for ChemicalComposition<'a> {
    #[inline]
    fn mul_assign(&mut self, other: i32) {
        match self {
            ChemicalComposition::Vec(c) => (c.mul_assign(other)),
            ChemicalComposition::Map(c) => (c.mul_assign(other)),
        };
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

impl<'transient, 'lifespan: 'transient> FromIterator<(&'lifespan ElementSpecification<'lifespan>, &'transient i32)> for ChemicalComposition<'lifespan> {
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
pub struct Iter<'inner, 'lifespan: 'inner> {
    composition: &'inner ChemicalComposition<'lifespan>,
    offest: usize
}


impl<'inner, 'lifespan: 'inner> Iterator for Iter<'inner, 'lifespan> {
    type Item = (&'inner ElementSpecification<'lifespan>, &'inner i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.composition {
            ChemicalComposition::Vec(c) => {
                if self.offest < self.composition.len() {
                    let item = (&c.get_ref()[self.offest].0, &c.get_ref()[self.offest].1);
                    self.offest += 1;
                    Some(item)
                } else {
                    None
                }
            },
            ChemicalComposition::Map(c) => {
                if let Some((c, v)) = c.iter().nth(self.offest) {
                    self.offest += 1;
                    Some((c, v))
                } else {
                    None
                }
            },
        }
    }
}


#[derive(Debug)]
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct IterMut<'inner, 'lifespan: 'inner> {
    composition: &'inner mut ChemicalComposition<'lifespan>,
    offest: usize
}


impl<'inner, 'lifespan: 'inner> Iterator for IterMut<'inner, 'lifespan> {
    type Item = (&'inner ElementSpecification<'lifespan>, &'inner mut i32);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.composition.len();
        let offset = self.offest;
        if offset >= n {
            return None
        }
        self.offest += 1;
        match self.composition {
            ChemicalComposition::Vec(c) => {
                todo!()
            },
            ChemicalComposition::Map(c) => {
                todo!()
            },
        }
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
