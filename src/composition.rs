use std::collections::hash_map::{HashMap, Iter};
use std::ops::{Index,Add, Sub, Mul, AddAssign, MulAssign, SubAssign};
use std::iter::{FromIterator};
use std::cmp;
use std::fmt;
use std::hash;
use std::convert;

use crate::element::{ Element };
use crate::table::PERIODIC_TABLE;

#[derive(Debug, Clone)]
pub struct ElementSpecification<'a> {
    pub element: &'a Element,
    pub isotope: u16
}

impl<'a> hash::Hash for ElementSpecification<'a> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl<'a> fmt::Display for ElementSpecification<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ElementSpecification({}, {})",
            self.element.symbol,
            self.isotope)
    }
}

impl<'a> ElementSpecification<'a> {
    pub fn new(element: &'a Element, isotope: u16) -> ElementSpecification<'a> {
        return ElementSpecification { element, isotope };
    }

    pub fn to_string(&self) -> String {
        if self.isotope == 0 {
            return format!("{}", self.element.symbol);
        } else {
            return format!("{}[{}]", self.element.symbol, self.isotope);
        }
    }

    pub fn parse(string: &str) -> Result<ElementSpecification, String> {
        let n = string.len();
        let elt_start = 0;
        let mut elt_end = n;
        let mut iso_start = n;
        let mut iso_end = n;
        for (i, c) in string.chars().enumerate() {
            if c == '[' {
                elt_end = i;
                if n > i {
                    iso_start = i + 1;
                } else {
                    return Err(String::from("Unclosed [ in element specifier"))
                }

            } else if c == ']' {
                iso_end = i;
            }
        }
        let elt_sym = &string[elt_start..elt_end];
        let element = &PERIODIC_TABLE[elt_sym];
        let isotope = if iso_start != iso_end {
            string[iso_start..iso_end].parse::<u16>().unwrap()
        } else {
            0
        };
        return Ok(ElementSpecification::new(element, isotope));
    }
}

impl<'a> cmp::PartialEq for ElementSpecification<'a> {
    fn eq(&self, other: &ElementSpecification) -> bool {
        if self.element != other.element {
            return false;
        }
        return self.isotope == other.isotope;
    }

    fn ne(&self, other: &ElementSpecification) -> bool {
        return !(self == other);
    }
}

impl<'a> cmp::Eq for ElementSpecification<'a> {}

impl<'a> convert::TryFrom<&'a str> for ElementSpecification<'a> {
    type Error = String;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        return match ElementSpecification::parse(string) {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }
}

/// Represents a collection of element-count pairs.
#[derive(Debug, Clone, Default)]
pub struct ChemicalComposition<'a> {
    pub composition: HashMap<ElementSpecification<'a>, i32>,
    pub mass_cache: Option<f64>
}

impl<'a> ChemicalComposition<'a> {
    pub fn new() -> ChemicalComposition<'a> {
        ChemicalComposition {..Default::default()}
    }

    pub fn from(elements: Vec<(&'a str, i32)>) -> ChemicalComposition<'a> {
        let composition: ChemicalComposition<'a> = elements.iter().cloned().collect();
        return composition;
    }

    pub fn calc_mass(&self) -> f64 {
        let mut total = 0.0;
        for (elt_spec, count) in &self.composition {
            let element = &elt_spec.element;
            total += if elt_spec.isotope == 0 {
                element.isotopes[&element.most_abundant_isotope].mass
            } else {
                element.isotopes[&elt_spec.isotope].mass
            } * (*count as f64);
        }
        return total;
    }

    pub fn mass(&self) -> f64 {
        let mass = match self.mass_cache {
            None => self.calc_mass(),
            Some(val) => val
        };
        return mass;
    }

    pub fn fmass(&mut self) -> f64 {
        let mass = match self.mass_cache {
            None => {
                let total = self.mass();
                self.mass_cache = Some(total);
                total
            },
            Some(val) => val
        };
        return mass;
    }

    pub fn get(&self, elt_spec: &ElementSpecification<'a>) -> i32 {
        return match self.composition.get(elt_spec) {
            Some(i) => *i,
            None => 0
        };
    }

    pub fn set(&mut self, elt_spec: ElementSpecification<'a>, count: i32) {
        self.composition.insert(elt_spec, count);
        self.mass_cache = None;
    }

    pub fn inc(&mut self, elt_spec: ElementSpecification<'a>, count: i32) {
        let i = self.get(&elt_spec);
        self.set(elt_spec, i + count);
    }

    pub fn iter(&self) -> Iter<ElementSpecification<'a>, i32> {
        return (self.composition).iter();
    }

    pub fn to_string(&self) -> String {
        let mut parts: Vec<(&ElementSpecification, &i32)> = self.composition.iter().collect();
        parts.sort_by_key(|elt_cnt| match elt_cnt.0.element.symbol.as_str() {
            "C" => 5001,
            "H" => 5000,
            _ => elt_cnt.0.element.most_abundant_mass as i64
        });
        parts.reverse();
        let tokens: Vec<String> = parts.iter().map(
            |elt_cnt| elt_cnt.0.to_string() + &(*(elt_cnt.1)).to_string()).collect();
        return tokens.join("");
    }

    fn _add_from(&mut self, other: &ChemicalComposition<'a>) {
        for (key, val) in other.composition.iter() {
            self.inc(key.clone(), *val);
        }
    }

    fn _sub_from(&mut self, other: &ChemicalComposition<'a>) {
        for (key, val) in other.composition.iter() {
            self.inc(key.clone(), -(*val));
        }
    }

    fn _mul_by(&mut self, scaler: i32) {
        let keys: Vec<ElementSpecification> = (&mut self.composition).keys().map(|e|e.clone()).collect();
        for key in keys {
            *(self.composition).entry(key).or_insert(0) *= scaler;
        }
    }
}

impl<'a> Index<&ElementSpecification<'a>> for ChemicalComposition<'a> {
    type Output = i32;

    fn index(&self, key: & ElementSpecification<'a>) -> &Self::Output {
        let ent = self.composition.get(key);
        return ent.unwrap();
    }
}

impl<'a> Add<&ChemicalComposition<'a>> for &ChemicalComposition<'a> {
    type Output = ChemicalComposition<'a>;

    fn add(self, other: &ChemicalComposition<'a>) -> Self::Output {
        let mut inst = self.clone();
        inst._add_from(other);
        return inst;
    }
}

impl<'a> Sub<&ChemicalComposition<'a>> for &ChemicalComposition<'a> {
    type Output = ChemicalComposition<'a>;

    fn sub(self, other: &ChemicalComposition<'a>) -> Self::Output {
        let mut inst = self.clone();
        inst._sub_from(other);
        return inst;
    }
}

impl<'a> Mul<i32> for &ChemicalComposition<'a> {
    type Output = ChemicalComposition<'a>;

    fn mul(self, other: i32) -> Self::Output {
        let mut inst = self.clone();
        inst._mul_by(other);
        return inst;
    }
}

impl<'a> AddAssign<&ChemicalComposition<'a>> for ChemicalComposition<'a> {
    fn add_assign(&mut self, other: &ChemicalComposition<'a>) {
        self._add_from(other);
    }
}

impl<'a> SubAssign<&ChemicalComposition<'a>> for ChemicalComposition<'a> {
    fn sub_assign(&mut self, other: &ChemicalComposition<'a>) {
        self._sub_from(other);
    }
}

impl<'a> MulAssign<i32> for ChemicalComposition<'a> {
    fn mul_assign(&mut self, other: i32) {
        self._mul_by(other);
    }
}

impl<'a> FromIterator<(ElementSpecification<'a>, i32)> for ChemicalComposition<'a> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (ElementSpecification<'a>, i32)> {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            composition.inc(k, v);
        }
        return composition;
    }
}

impl<'a> FromIterator<(&'a str, i32)> for ChemicalComposition<'a> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (&'a str, i32)> {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            let elt_spec = ElementSpecification::parse(k).unwrap();
            composition.inc(elt_spec, v);
        }
        return composition;
    }
}
