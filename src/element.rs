use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::ops;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use fnv::FnvBuildHasher as RandomState;

type NeutronShiftType = i8;
type ElementNumberType = u8;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/** A known isotope of an element with a known number of neutrons,
mass, and relative abundance
*/
pub struct Isotope {
    pub mass: f64,
    pub abundance: f64,
    pub neutrons: u16,
    pub neutron_shift: NeutronShiftType,
}

impl fmt::Display for Isotope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Isotope({}, {}, {}, {})",
            self.mass, self.abundance, self.neutrons, self.neutron_shift
        )
    }
}

impl hash::Hash for Isotope {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.neutrons.hash(state);
    }
}

impl cmp::PartialEq<Isotope> for Isotope {
    fn eq(&self, other: &Isotope) -> bool {
        if (self.mass - other.mass).abs() > 1e-3
            || (self.abundance - other.abundance).abs() > 1e-3
            || self.neutrons != other.neutrons
            || self.neutron_shift != other.neutron_shift
        {
            return false;
        }
        true
    }
}

impl cmp::PartialOrd<Isotope> for Isotope {
    fn partial_cmp(&self, other: &Isotope) -> Option<cmp::Ordering> {
        self.mass.partial_cmp(&other.mass)
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/** A chemical element with known masses and isotopic frequency.

This type forms the foundation of the library, and is *usually*
treated like a singleton in a [`PeriodicTable`].
*/
pub struct Element {
    pub symbol: String,
    pub isotopes: HashMap<u16, Isotope, RandomState>,
    pub most_abundant_isotope: u16,
    pub most_abundant_mass: f64,
    pub min_neutron_shift: NeutronShiftType,
    pub max_neutron_shift: NeutronShiftType,
    pub element_number: ElementNumberType,
}

impl Element {
    pub fn mass(&self) -> f64 {
        self.isotopes[&self.most_abundant_isotope].mass
    }

    pub fn calc_min_neutron_shift(&self) -> NeutronShiftType {
        if self.min_neutron_shift != 0 {
            return self.min_neutron_shift;
        }
        self.isotopes
            .values()
            .map(|iso| iso.neutron_shift)
            .min()
            .unwrap_or(0)
    }

    pub fn calc_max_neutron_shift(&self) -> NeutronShiftType {
        if self.max_neutron_shift != 0 {
            return self.max_neutron_shift;
        }
        self.isotopes
            .values()
            .map(|iso| iso.neutron_shift)
            .max()
            .unwrap_or(0)
    }

    pub fn isotope_by_shift(&self, shift: NeutronShiftType) -> Option<&Isotope> {
        let num = self.most_abundant_isotope as i16 + shift as i16;
        self.isotopes.get(&(num as u16))
    }

    pub fn index_isotopes(&mut self) {
        self.max_neutron_shift = 0;
        self.min_neutron_shift = 0;
        self.max_neutron_shift = self.calc_max_neutron_shift();
        self.min_neutron_shift = self.calc_min_neutron_shift();
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Element({}, {}, {})",
            self.symbol,
            self.isotopes[&self.most_abundant_isotope],
            self.isotopes.len()
        )
    }
}

impl hash::Hash for Element {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

impl cmp::PartialEq<Element> for Element {
    #[inline]
    fn eq(&self, other: &Element) -> bool {
        if self.symbol != other.symbol || self.most_abundant_isotope != other.most_abundant_isotope
        {
            return false;
        }
        true
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/** A mapping connecting [`Element`] to its textual symbol.

This type is referenced indirectly through all other structures
that depend upon [`Element`] or [`ChemicalComposition`](crate::ChemicalComposition).

A global `lazy_static` constant is available as `PERIODIC_TABLE`.
*/
pub struct PeriodicTable {
    pub elements: HashMap<String, Element, RandomState>,
}

impl PeriodicTable {
    pub fn new() -> PeriodicTable {
        PeriodicTable {
            ..Default::default()
        }
    }

    pub fn add(&mut self, element: Element) {
        self.elements.insert(element.symbol.clone(), element);
    }

    pub fn get(&self, symbol: &str) -> Option<&Element> {
        self.elements.get(symbol)
    }
}

impl ops::Index<&str> for PeriodicTable {
    type Output = Element;

    #[inline]
    fn index(&self, i: &str) -> &Self::Output {
        &self.elements[i]
    }
}
