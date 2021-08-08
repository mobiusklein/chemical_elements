use std::collections::hash_map::{HashMap, Iter};
use std::ops::{Index,Add, Sub, Mul, AddAssign, MulAssign, SubAssign};
use std::iter::{FromIterator};
use std::str::FromStr;
use std::cmp;
use std::fmt;
use std::hash;
use std::convert;

use crate::element::{ Element, PeriodicTable };
use crate::table::{PERIODIC_TABLE};
use crate::formula::{parse_formula, parse_formula_with_table, FormulaParserError};


#[derive(Debug, Clone, Copy)]
pub enum ElementSpecificationParsingError {
    UnclosedIsotope,

}

#[derive(Debug, Clone)]
pub struct ElementSpecification<'element> {
    pub element: &'element Element,
    pub isotope: u16
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


impl<'element> hash::Hash for ElementSpecification<'element> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl<'element> fmt::Display for ElementSpecification<'element> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ElementSpecification({}, {})",
            self.element.symbol,
            self.isotope)
    }
}

impl<'transient, 'lifespan: 'transient, 'element> ElementSpecification<'element> {
    pub fn new(element: &'element Element, isotope: u16) -> ElementSpecification<'element> {
        return ElementSpecification { element, isotope };
    }

    pub fn to_string(&self) -> String {
        if self.isotope == 0 {
            return format!("{}", self.element.symbol);
        } else {
            return format!("{}[{}]", self.element.symbol, self.isotope);
        }
    }

    pub fn parse(string: &'transient str) -> Result<ElementSpecification<'lifespan>, ElementSpecificationParsingError> {
        Self::parse_with(string, &PERIODIC_TABLE)
    }

    pub fn parse_with(string: &'transient str, periodic_table: &'lifespan PeriodicTable) -> Result<ElementSpecification<'lifespan>, ElementSpecificationParsingError> {
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
                    return Err(ElementSpecificationParsingError::UnclosedIsotope)
                }

            } else if c == ']' {
                iso_end = i;
            }
        }
        let elt_sym = &string[elt_start..elt_end];
        let element = &periodic_table[elt_sym];
        let isotope = if iso_start != iso_end {
            string[iso_start..iso_end].parse::<u16>().unwrap()
        } else {
            0
        };
        return Ok(ElementSpecification::new(element, isotope));
    }
}

impl<'a> convert::TryFrom<&str> for ElementSpecification<'a> {
    type Error = ElementSpecificationParsingError;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        return match ElementSpecification::parse(string) {
            Ok(r) => Ok(r),
            Err(e) => Err(e)
        }
    }
}

impl<'a> FromStr for ElementSpecification<'a> {
    type Err = ElementSpecificationParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match ElementSpecification::parse(s) {
            Ok(r) => Ok(r),
            Err(err) => Err(err)
        }
    }
}

/// Represents a collection of element-count pairs.
#[derive(Debug, Clone, Default)]
pub struct ChemicalComposition<'a> {
    pub composition: HashMap<ElementSpecification<'a>, i32>,
    pub mass_cache: Option<f64>
}


impl<'lifespan, 'transient, 'outer: 'transient> ChemicalComposition<'lifespan> {
    pub fn new() -> ChemicalComposition<'lifespan> {
        ChemicalComposition {..Default::default()}
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

    pub fn get(&self, elt_spec: &ElementSpecification<'lifespan>) -> i32 {
        return match self.composition.get(elt_spec) {
            Some(i) => *i,
            None => 0
        };
    }

    pub fn set(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        self.composition.insert(elt_spec, count);
        self.mass_cache = None;
    }

    pub fn inc(&mut self, elt_spec: ElementSpecification<'lifespan>, count: i32) {
        let i = self.get(&elt_spec);
        self.set(elt_spec, i + count);
    }

    pub fn iter(&self) -> Iter<ElementSpecification<'lifespan>, i32> {
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

    pub fn _add_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.composition.iter() {
            self.inc(key.clone(), *val);
        }
    }

    pub fn _sub_from(&'outer mut self, other: &'transient ChemicalComposition<'lifespan>) {
        for (key, val) in other.composition.iter() {
            let newkey: ElementSpecification<'lifespan> = key.clone();
            self.inc(newkey, -(*val));
        }
    }

    fn _mul_by(&mut self, scaler: i32) {
        let keys: Vec<ElementSpecification> = (&mut self.composition).keys().map(|e|e.clone()).collect();
        for key in keys {
            *(self.composition).entry(key).or_insert(0) *= scaler;
        }
    }

    pub fn len(&self) -> usize {
        self.composition.len()
    }

    pub fn parse(string: &str) -> Result<ChemicalComposition, FormulaParserError> {
        parse_formula(string)
    }

    pub fn parse_with(string: &str, periodic_table: &'lifespan PeriodicTable) -> Result<ChemicalComposition<'lifespan>, FormulaParserError> {
        parse_formula_with_table(string, periodic_table)
    }
}

impl<'lifespan> Index<&ElementSpecification<'lifespan>> for ChemicalComposition<'lifespan> {
    type Output = i32;

    fn index(&self, key: & ElementSpecification<'lifespan>) -> &Self::Output {
        let ent = self.composition.get(key);
        return ent.unwrap();
    }
}

impl<'lifespan> PartialEq<ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
    fn eq(&self, other: &ChemicalComposition<'lifespan>) -> bool {
        self.composition == other.composition
    }

    fn ne(&self, other: &ChemicalComposition<'lifespan>) -> bool {
        !(self.composition == other.composition)
    }
}


impl<'lifespan> Add<&ChemicalComposition<'lifespan>> for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    fn add(self, other: &ChemicalComposition<'lifespan>) -> Self::Output {
        let mut inst = self.clone();
        inst._add_from(other);
        return inst;
    }
}

impl<'lifespan> Sub<&'lifespan ChemicalComposition<'_>> for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    fn sub(self, other: &'lifespan ChemicalComposition<'_>) -> Self::Output {
        let mut inst = self.clone();
        inst._sub_from(other);
        return inst;
    }
}

impl<'lifespan> Mul<i32> for &ChemicalComposition<'lifespan> {
    type Output = ChemicalComposition<'lifespan>;

    fn mul(self, other: i32) -> Self::Output {
        let mut inst = self.clone();
        inst._mul_by(other);
        return inst;
    }
}

impl<'lifespan> AddAssign<&ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
    fn add_assign(&mut self, other: &ChemicalComposition<'lifespan>) {
        self._add_from(other);
    }
}

impl<'lifespan> SubAssign<&'_ ChemicalComposition<'lifespan>> for ChemicalComposition<'lifespan> {
    fn sub_assign(&mut self, other: &'_ ChemicalComposition<'lifespan>) {
        self._sub_from(other);
    }
}

impl<'lifespan> MulAssign<i32> for ChemicalComposition<'_> {
    fn mul_assign(&mut self, other: i32) {
        self._mul_by(other);
    }
}

impl<'lifespan> FromIterator<(ElementSpecification<'lifespan>, i32)> for ChemicalComposition<'lifespan> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (ElementSpecification<'lifespan>, i32)> {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            composition.inc(k, v);
        }
        return composition;
    }
}

impl<'lifespan> FromIterator<(&'lifespan str, i32)> for ChemicalComposition<'lifespan> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (&'lifespan str, i32)> {
        let mut composition = ChemicalComposition::new();
        for (k, v) in iter {
            let elt_spec = ElementSpecification::parse(k).unwrap();
            composition.inc(elt_spec, v);
        }
        return composition;
    }
}


impl<'lifespan> convert::From<Vec<(&'lifespan str, i32)>> for ChemicalComposition<'lifespan> {
    fn from(elements: Vec<(&'lifespan str, i32)>) -> Self {
        let composition: ChemicalComposition<'lifespan> = elements.iter().cloned().collect();
        return composition;
    }
}

impl<'lifespan> convert::From<Vec<(ElementSpecification<'lifespan>, i32)>> for ChemicalComposition<'lifespan> {
    fn from(elements: Vec<(ElementSpecification<'lifespan>, i32)>) -> Self {
        let composition: ChemicalComposition<'lifespan> = elements.iter().cloned().collect();
        return composition;
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::convert::{TryFrom, From};

    #[test]
    fn test_parse() {
        let case = ChemicalComposition::parse("H2O").expect("Failed to parse");
        let mut ctrl = ChemicalComposition::new();
        ctrl.set(ElementSpecification::try_from("O").unwrap(), 1);
        ctrl.set(ElementSpecification::try_from("H").unwrap(), 2);
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
        ctrl.set(ElementSpecification::try_from("O").unwrap(), 1);
        ctrl.set(ElementSpecification::try_from("H").unwrap(), 2);
        assert_eq!(case, ctrl);
    }

    #[test]
    fn test_from_vec_elt_spec() {
        let hydrogen = ElementSpecification::try_from("H").unwrap();
        let oxygen = ElementSpecification::try_from("O").unwrap();
        let case = ChemicalComposition::from(vec![(oxygen, 1), (hydrogen, 2)]);
        let mut ctrl = ChemicalComposition::new();

        let hydrogen = ElementSpecification::try_from("H").unwrap();
        let oxygen = ElementSpecification::try_from("O").unwrap();
        ctrl.set(ElementSpecification::try_from(oxygen).unwrap(), 1);
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
    fn test_mul() {
        let case = ChemicalComposition::from(vec![("O", 1), ("H", 2)]);
        let ctrl = ChemicalComposition::from(vec![("O", 2), ("H", 4)]);

        let combo = &case * 2;
        assert_eq!(ctrl, combo);
    }
}
