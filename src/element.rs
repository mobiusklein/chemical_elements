use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::ops;

#[cfg(feature = "serde1")]
use serde::{Serialize, Deserialize};

use fnv::FnvBuildHasher as RandomState;

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
/** A known isotope of an element with a known number of neutrons,
mass, and relative abundance
*/
pub struct Isotope {
    pub mass: f64,
    pub abundance: f64,
    pub neutrons: u16,
    pub neutron_shift: i8,
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
        if (self.mass - other.mass).abs() > 1e-3 {
            return false;
        } else if (self.abundance - other.abundance).abs() > 1e-3 {
            return false;
        } else if self.neutrons != other.neutrons {
            return false;
        } else if self.neutron_shift != other.neutron_shift {
            return false;
        }
        return true;
    }

    fn ne(&self, other: &Isotope) -> bool {
        return !(self == other);
    }
}

impl cmp::PartialOrd<Isotope> for Isotope {
    fn partial_cmp(&self, other: &Isotope) -> Option<cmp::Ordering> {
        return self.mass.partial_cmp(&other.mass);
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
/** A chemical element with known masses and isotopic frequency.

This type forms the foundation of the library, and is *usually*
treated like a singleton in a [`PeriodicTable`].
*/
pub struct Element {
    pub symbol: String,
    pub isotopes: HashMap<u16, Isotope, RandomState>,
    pub most_abundant_isotope: u16,
    pub most_abundant_mass: f64,
    pub min_neutron_shift: i8,
    pub max_neutron_shift: i8,
    pub element_number: u8,
}

impl Element {
    pub fn mass(&self) -> f64 {
        return self.isotopes[&self.most_abundant_isotope].mass;
    }

    pub fn calc_min_neutron_shift(&self) -> i8 {
        if self.min_neutron_shift != 0 {
            return self.min_neutron_shift;
        }
        match self.isotopes.values().map(|iso| iso.neutron_shift).min() {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn calc_max_neutron_shift(&self) -> i8 {
        if self.max_neutron_shift != 0 {
            return self.max_neutron_shift;
        }
        match self.isotopes.values().map(|iso| iso.neutron_shift).max() {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn isotope_by_shift(&self, shift: i8) -> Option<&Isotope> {
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
        if self.symbol != other.symbol {
            return false;
        } else if self.most_abundant_isotope != other.most_abundant_isotope {
            return false;
        }
        return true;
    }

    fn ne(&self, other: &Element) -> bool {
        return !(self == other);
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature="serde1", derive(Serialize, Deserialize))]
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
        return PeriodicTable {
            ..Default::default()
        };
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
        return &self.elements[i];
    }
}
