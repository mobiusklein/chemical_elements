use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};

use crate::abstract_composition;
use crate::element_specification::ElementSpecification;
use crate::composition_map::ChemicalCompositionMap as ChemicalCompositionMap;
use crate::composition_list::ChemicalCompositionVec;
use crate::abstract_composition::ChemicalComposition as AbstractChemicalComposition;

pub trait ChemicalCompositionLike<'inner, 'lifespan: 'inner> {
    type Iter: Iterator<Item = (&'inner ElementSpecification<'lifespan>, &'inner i32)>;

    /// Access a specific element's count, or `0` if that element is absent
    /// from the composition
    fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32;

    /// Set the count for a specific element. This will invalidate the mass cache.
    fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32);

    /// Add some value to the count of the specified element. This will invalidate the
    /// mass cache.
    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        let mut val = self.get(&elt_spec);
        val += count;
        self.set(elt_spec, val);
    }

    /**
    # Mass calculation Methods

    [`ChemicalComposition`] has three methods for computing the monoisotopic
    mass of the composition it represents to handle mutability.
    */

    /**
    Get the mass of this chemical composition. If the mass cache
    has been populated, return that instead of repeating the calculation.
    */
    fn mass(&self) -> f64;

    /**
    Get the mass of this chemical composition, and cache it,
    or reuse the cached value. This requires mutability, so this method
    must be called explicitly.
    */
    fn fmass(&mut self) -> f64 {
        self.mass()
    }

    fn is_empty(&self) -> bool;

    fn len(&self) -> usize;

    fn _iter(&'inner self) -> Self::Iter;
    // fn _iter_mut(&'inner mut self) -> dyn Iterator<Item = (&'inner ElementSpecification<'lifespan>, &'inner mut i32)>;

    fn _mul_by(&mut self, scaler: i32);
}

#[derive(Debug)]
pub struct VecIt<'transient, 'lifespan: 'transient> {
    composition: &'transient ChemicalCompositionVec<'lifespan>,
    offset: usize,
}

impl<'transient, 'lifespan: 'transient> Iterator for VecIt<'transient, 'lifespan> {
    type Item = (&'transient ElementSpecification<'lifespan>, &'transient i32);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.composition.len();
        let offset = self.offset;
        if n <= offset {
            return None;
        }
        self.offset += 1;
        self.composition
            .iter()
            .nth(offset)
            .and_then(|(k, v)| Some((k, v)))
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for ChemicalCompositionVec<'lifespan>
{
    type Iter = VecIt<'transient, 'lifespan>;

    fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        self.get(elt_spec)
    }

    fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.set(elt_spec, count)
    }

    fn mass(&self) -> f64 {
        self.mass()
    }

    fn fmass(&mut self) -> f64 {
        self.fmass()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn _iter(&'transient self) -> Self::Iter {
        VecIt {
            composition: &self,
            offset: 0,
        }
    }

    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.inc(elt_spec, count)
    }

    fn _mul_by(&mut self, scaler: i32) {
        (*self) *= scaler;
    }
}

#[derive(Debug)]
pub struct MapIt<'transient, 'lifespan: 'transient> {
    composition: &'transient ChemicalCompositionMap<'lifespan>,
    offset: usize,
}

impl<'transient, 'lifespan: 'transient> Iterator for MapIt<'transient, 'lifespan> {
    type Item = (&'transient ElementSpecification<'lifespan>, &'transient i32);

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.composition.len();
        let offset = self.offset;
        if n <= offset {
            return None;
        }
        self.offset += 1;
        self.composition
            .iter()
            .nth(offset)
            .and_then(|(k, v)| Some((k, v)))
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for ChemicalCompositionMap<'lifespan>
{
    type Iter = MapIt<'transient, 'lifespan>;

    fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        self.get(elt_spec)
    }

    fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.set(elt_spec, count)
    }

    fn mass(&self) -> f64 {
        self.mass()
    }

    fn fmass(&mut self) -> f64 {
        self.fmass()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn _iter(&'transient self) -> Self::Iter {
        MapIt {
            composition: &self,
            offset: 0,
        }
    }

    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.inc(elt_spec, count)
    }

    fn _mul_by(&mut self, scaler: i32) {
        *self *= scaler;
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for AbstractChemicalComposition<'lifespan>
{
    type Iter = abstract_composition::Iter<'transient, 'lifespan>;

    fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        self.get(elt_spec)
    }

    fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.set(elt_spec, count)
    }

    fn mass(&self) -> f64 {
        self.mass()
    }

    fn fmass(&mut self) -> f64 {
        self.fmass()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn _iter(&'transient self) -> Self::Iter {
        self.iter()
    }

    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.inc(elt_spec, count)
    }

    fn _mul_by(&mut self, scaler: i32) {
        match self {
            AbstractChemicalComposition::Vec(c) => {
                *c *= scaler;
            },
            AbstractChemicalComposition::Map(c) => {
                *c *= scaler;
            },
        }
    }
}

macro_rules! impl_from {
    ($frm:ty, $to:ty) => {
        impl<'lifespan> From<$frm> for $to {
            fn from(value: $frm) -> Self {
                let mut inst = Self::default();
                value.iter().for_each(|(k, v)| {
                    inst.set(*k, *v);
                });
                inst
            }
        }
    };
}

impl_from!(ChemicalCompositionMap<'lifespan>, ChemicalCompositionVec<'lifespan>);
impl_from!(ChemicalCompositionVec<'lifespan>, ChemicalCompositionMap<'lifespan>);
impl_from!(AbstractChemicalComposition<'lifespan>, ChemicalCompositionVec<'lifespan>);
impl_from!(AbstractChemicalComposition<'lifespan>, ChemicalCompositionMap<'lifespan>);
impl_from!(ChemicalCompositionVec<'lifespan>, AbstractChemicalComposition<'lifespan>);
impl_from!(ChemicalCompositionMap<'lifespan>, AbstractChemicalComposition<'lifespan>);

macro_rules! impl_arithmetic {
    ($tp:ty) => {
        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            Add<&'inner C> for &$tp
        {
            type Output = $tp;

            #[inline]
            fn add(self, other: &'inner C) -> Self::Output {
                let mut inst = self.clone();
                other._iter().for_each(|(k, v)| {
                    inst.inc(*k, *v);
                });
                return inst;
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            Sub<&'inner C> for &$tp
        {
            type Output = $tp;

            #[inline]
            fn sub(self, other: &'inner C) -> Self::Output {
                let mut inst = self.clone();
                other._iter().for_each(|(k, v)| {
                    inst.inc(*k, -*v);
                });
                return inst;
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            Add<&'inner C> for $tp
        {
            type Output = $tp;

            #[inline]
            fn add(self, other: &'inner C) -> Self::Output {
                let mut inst = self.clone();
                other._iter().for_each(|(k, v)| {
                    inst.inc(*k, *v);
                });
                return inst;
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            Sub<&'inner C> for $tp
        {
            type Output = $tp;

            #[inline]
            fn sub(self, other: &'inner C) -> Self::Output {
                let mut inst = self.clone();
                other._iter().for_each(|(k, v)| {
                    inst.inc(*k, -*v);
                });
                return inst;
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            AddAssign<&'inner C> for &mut $tp
        {
            #[inline]
            fn add_assign(&mut self, other: &'inner C) {
                other._iter().for_each(|(k, v)| {
                    self.inc(*k, *v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            SubAssign<&'inner C> for &mut $tp
        {
            #[inline]
            fn sub_assign(&mut self, other: &'inner C) {
                other._iter().for_each(|(k, v)| {
                    self.inc(*k, -*v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            AddAssign<&'inner C> for $tp
        {
            #[inline]
            fn add_assign(&mut self, other: &'inner C) {
                other._iter().for_each(|(k, v)| {
                    self.inc(*k, *v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            SubAssign<&'inner C> for $tp
        {
            #[inline]
            fn sub_assign(&mut self, other: &'inner C) {
                other._iter().for_each(|(k, v)| {
                    self.inc(*k, -*v);
                });
            }
        }

        impl<'lifespan> Mul<i32> for &$tp {
            type Output = $tp;

            #[inline]
            fn mul(self, other: i32) -> Self::Output {
                let mut inst = self.clone();
                inst._mul_by(other);
                return inst;
            }
        }

        impl<'lifespan> Mul<i32> for $tp {
            type Output = $tp;

            #[inline]
            fn mul(self, other: i32) -> Self::Output {
                let mut inst = self.clone();
                inst._mul_by(other);
                return inst;
            }
        }

        impl<'lifespan> MulAssign<i32> for $tp {
            #[inline]
            fn mul_assign(&mut self, other: i32) {
                self._mul_by(other);
            }
        }

        impl<'lifespan> MulAssign<i32> for &mut $tp {
            #[inline]
            fn mul_assign(&mut self, other: i32) {
                self._mul_by(other);
            }
        }

        impl<'lifespan> Neg for $tp {
            type Output = $tp;

            #[inline]
            fn neg(mut self) -> Self::Output {
                self._mul_by(-1);
                self
            }
        }

        impl<'lifespan> Neg for &$tp {
            type Output = $tp;

            #[inline]
            fn neg(self) -> Self::Output {
                let mut dup = self.clone();
                dup._mul_by(-1);
                dup
            }
        }
    };

}

impl_arithmetic!(ChemicalCompositionMap<'lifespan>);
impl_arithmetic!(ChemicalCompositionVec<'lifespan>);
impl_arithmetic!(AbstractChemicalComposition<'lifespan>);

impl<'inner, 'lifespan: 'inner> IntoIterator for &'inner ChemicalCompositionMap<'lifespan> {
    type IntoIter =
        <ChemicalCompositionMap<'lifespan> as ChemicalCompositionLike<'inner, 'lifespan>>::Iter;
    type Item = <<ChemicalCompositionMap<'lifespan> as ChemicalCompositionLike<'inner, 'lifespan>>::Iter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self._iter()
    }
}

impl<'inner, 'lifespan: 'inner> IntoIterator for &'inner ChemicalCompositionVec<'lifespan> {
    type IntoIter =
        <ChemicalCompositionVec<'lifespan> as ChemicalCompositionLike<'inner, 'lifespan>>::Iter;
    type Item = <<ChemicalCompositionVec<'lifespan> as ChemicalCompositionLike<'inner, 'lifespan>>::Iter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self._iter()
    }
}

impl<'inner, 'lifespan: 'inner> IntoIterator for &'inner AbstractChemicalComposition<'lifespan> {
    type IntoIter = <AbstractChemicalComposition<'lifespan> as ChemicalCompositionLike<
        'inner,
        'lifespan,
    >>::Iter;
    type Item = <<AbstractChemicalComposition<'lifespan> as ChemicalCompositionLike<
        'inner,
        'lifespan,
    >>::Iter as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self._iter()
    }
}

pub trait ChemicalCompositionBehavior<'inner, 'lifespan: 'inner>:
    ChemicalCompositionLike<'inner, 'lifespan> + Default
where
    &'inner Self: IntoIterator + 'inner,
    &'inner Self: Add<&'inner Self>,
    &'inner Self: Sub<&'inner Self>,
    &'inner mut Self: AddAssign<&'inner Self>,
    &'inner mut Self: SubAssign<&'inner Self>,
    Self: AddAssign<&'inner Self>,
    Self: SubAssign<&'inner Self>,
    Self: MulAssign<i32>,
    &'inner Self: Mul<i32>,
{
}

impl<'inner, 'lifespan: 'inner> ChemicalCompositionBehavior<'inner, 'lifespan>
    for ChemicalCompositionMap<'lifespan>
{
}
impl<'inner, 'lifespan: 'inner> ChemicalCompositionBehavior<'inner, 'lifespan>
    for ChemicalCompositionVec<'lifespan>
{
}
impl<'inner, 'lifespan: 'inner> ChemicalCompositionBehavior<'inner, 'lifespan>
    for AbstractChemicalComposition<'lifespan>
{
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let comp = AbstractChemicalComposition::parse("C6H12O6").unwrap();
        let mut parts = 0;
        for (_, v) in &comp {
            parts += *v;
        }
        assert_eq!(24, parts)
    }
}
