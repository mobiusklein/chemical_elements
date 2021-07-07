use std::cmp;
use std::collections::HashMap;
use std::fmt;
use std::hash;
use std::ops;

#[derive(Debug, Clone, Default)]
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
pub struct Element {
    pub symbol: String,
    pub isotopes: HashMap<u16, Isotope>,
    pub most_abundant_isotope: u16,
    pub most_abundant_mass: f64,
}

impl Element {
    pub fn mass(&self) -> f64 {
        return self.isotopes[&self.most_abundant_isotope].mass;
    }

    pub fn min_neutron_shift(&self) -> i8 {
        match self.isotopes.values().map(|iso| iso.neutron_shift).min() {
            Some(i) => i,
            None => 0,
        }
    }

    pub fn max_neutron_shift(&self) -> i8 {
        match self.isotopes.values().map(|iso| iso.neutron_shift).max() {
            Some(i) => i,
            None => 0,
        }
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
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
    }
}

impl cmp::PartialEq<Element> for Element {
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
pub struct PeriodicTable {
    pub elements: HashMap<String, Element>,
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
}

impl ops::Index<&String> for PeriodicTable {
    type Output = Element;

    fn index(&self, i: &String) -> &Self::Output {
        return &self.elements[i];
    }
}

impl ops::Index<&str> for PeriodicTable {
    type Output = Element;

    fn index(&self, i: &str) -> &Self::Output {
        return &self.elements[i];
    }
}
