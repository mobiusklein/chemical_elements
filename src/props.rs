use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Neg};

use crate::element_specification::ElementSpecification;
use crate::composition_map::ChemicalCompositionMap as ChemicalCompositionMap;
use crate::composition_list::ChemicalCompositionVec;
use crate::abstract_composition::{ChemicalComposition as AbstractChemicalComposition, Iter as AbstractIter, IterMut as AbstractIterMut};

pub trait ChemicalCompositionLike<'inner, 'lifespan: 'inner> {

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

    [`ChemicalCompositionLike`] has three methods for computing the monoisotopic
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

    fn iter(&self) -> AbstractIter<'_, 'lifespan>;

    fn iter_mut(&mut self) -> AbstractIterMut<'_, 'lifespan>;

    fn _mul_by(&mut self, scaler: i32) {
        for (_, v)  in self.iter_mut() {
            *v *= scaler;
        }
    }
}


impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for ChemicalCompositionVec<'lifespan>
{

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

    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.inc(elt_spec, count)
    }

    fn _mul_by(&mut self, scaler: i32) {
        (*self) *= scaler;
    }

    fn iter(&self) -> AbstractIter<'_, 'lifespan> {
        AbstractIter::Vec(self.iter())
    }

    fn iter_mut(&mut self) -> AbstractIterMut<'_, 'lifespan> {
        AbstractIterMut::Vec(self.iter_mut())
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for ChemicalCompositionMap<'lifespan>
{

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

    fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.inc(elt_spec, count)
    }

    fn _mul_by(&mut self, scaler: i32) {
        *self *= scaler;
    }

    fn iter(&self) -> AbstractIter<'_, 'lifespan> {
        AbstractIter::Map(self.iter())
    }

    fn iter_mut(&mut self) -> AbstractIterMut<'_, 'lifespan> {
        AbstractIterMut::Map(self.iter_mut())
    }
}

impl<'transient, 'lifespan: 'transient> ChemicalCompositionLike<'transient, 'lifespan>
    for AbstractChemicalComposition<'lifespan>
{

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

    fn iter(&self) -> AbstractIter<'_, 'lifespan> {
        self.iter()
    }

    fn iter_mut(&mut self) -> AbstractIterMut<'_, 'lifespan> {
        self.iter_mut()
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
                other.iter().for_each(|(k, v)| {
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
                other.iter().for_each(|(k, v)| {
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
                other.iter().for_each(|(k, v)| {
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
                other.iter().for_each(|(k, v)| {
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
                other.iter().for_each(|(k, v)| {
                    self.inc(*k, *v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            SubAssign<&'inner C> for &mut $tp
        {
            #[inline]
            fn sub_assign(&mut self, other: &'inner C) {
                other.iter().for_each(|(k, v)| {
                    self.inc(*k, -*v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            AddAssign<&'inner C> for $tp
        {
            #[inline]
            fn add_assign(&mut self, other: &'inner C) {
                other.iter().for_each(|(k, v)| {
                    self.inc(*k, *v);
                });
            }
        }

        impl<'inner, 'lifespan: 'inner, C: ChemicalCompositionLike<'inner, 'lifespan>>
            SubAssign<&'inner C> for $tp
        {
            #[inline]
            fn sub_assign(&mut self, other: &'inner C) {
                other.iter().for_each(|(k, v)| {
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
    type IntoIter = AbstractIter<'inner, 'lifespan>;
    type Item = <AbstractIter<'inner, 'lifespan> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        AbstractIter::Map(self.iter())
    }
}

impl<'inner, 'lifespan: 'inner> IntoIterator for &'inner ChemicalCompositionVec<'lifespan> {
    type IntoIter = AbstractIter<'inner, 'lifespan>;
    type Item = <AbstractIter<'inner, 'lifespan> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        AbstractIter::Vec(self.iter())
    }
}

impl<'inner, 'lifespan: 'inner> IntoIterator for &'inner AbstractChemicalComposition<'lifespan> {
    type IntoIter = AbstractIter<'inner, 'lifespan>;
    type Item = <AbstractIter<'inner, 'lifespan> as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[allow(unused)]
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
