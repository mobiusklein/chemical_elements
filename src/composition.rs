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
pub struct ElementSpecification<'element> {
    pub element: &'element Element,
    pub isotope: u16
}

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

impl<'element> ElementSpecification<'element> {
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

#[derive(Debug)]
enum FormulaParserState {
    New,
    Element,
    Isotope,
    IsotopeToCount,
    Count,
    Group,
    GroupToGroupCount,
    GroupCount
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

    // TODO: Move to stateful parser struct
    #[allow(unused)]
    pub fn parse(string: &str) -> Result<ChemicalComposition, &str> {
        let mut state = FormulaParserState::New;
        let mut acc = ChemicalComposition::new();

        let mut element_start: usize = 0;
        let mut element_end: usize = 0;
        let mut isotope_start: usize = 0;
        let mut isotope_end: usize = 0;
        let mut count_start: usize = 0;
        let mut count_end: usize = 0;
        let mut paren_stack: i32 = 0;
        let mut group_start: usize = 0;
        let mut group_end: usize = 0;
        let mut group_count_start: usize = 0;
        let mut group_count_end: usize = 0;

        let n = string.len();

        for (i, c) in string.char_indices() {
            match state {
                FormulaParserState::New => {
                    if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                        element_start = i;
                        state = FormulaParserState::Element;
                    } else if c == '(' {
                        paren_stack += 1;
                        group_start = i;
                        state = FormulaParserState::Group;
                    } else {
                        return Err("Invalid start of formula");
                    }
                },
                FormulaParserState::Group => {
                    if c == ')' {
                        paren_stack -= 1;
                        if paren_stack == 0 {
                            group_end = i;
                            state = FormulaParserState::GroupToGroupCount;
                        }
                    } else if c == '(' {
                        paren_stack += 1;
                    }
                },
                FormulaParserState::Element => {
                    if c.is_ascii_alphabetic() {
                        if c.is_uppercase() {
                            element_end = i;
                            let elt_sym = &string[element_start..element_end];
                            let elt = &PERIODIC_TABLE[elt_sym];
                            let elt_spec = ElementSpecification {
                                element: elt,
                                isotope: 0
                            };
                            acc.inc(elt_spec, 1);
                            state = FormulaParserState::Element;
                            element_start = i;
                            element_end = 0;
                        }
                    } else if c.is_numeric() {
                        element_end = i;
                        count_start = i;
                        state = FormulaParserState::Count;
                    } else if c == '[' {
                        isotope_start = i + 1;
                        state = FormulaParserState::Isotope;
                    } else if c == '(' {
                        element_end = i;
                        let elt_sym = &string[element_start..element_end];
                        let elt = &PERIODIC_TABLE[elt_sym];
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: 0
                        };
                        acc.inc(elt_spec, 1);
                        element_start = 0;
                        element_end = 0;
                        paren_stack += 1;
                        group_start = i;
                        state = FormulaParserState::Group;

                    }
                },
                FormulaParserState::Isotope => {
                    if c == ']' {
                        isotope_end = i;
                        state = FormulaParserState::IsotopeToCount;
                    } else if !c.is_numeric() {
                        return Err("Invalid non-numeric character in isotope")
                    }
                },
                FormulaParserState::Count => {
                    if !c.is_numeric() {
                        count_end = i;
                        let count_parse = &string[count_start..count_end].parse::<i32>();
                        let count: i32 = match count_parse {
                            Ok(val) => {*val},
                            Err(msg) => {return Err("Failed to parse integer from element count");}
                        };
                        let isotope: u16 = if isotope_end != isotope_start {
                            match &string[isotope_start..isotope_end].parse::<u16>() {
                                Ok(val) => {*val},
                                Err(msg) => {return Err("Failed to parse integer from isotope count");}
                            }
                        } else {
                            0
                        };
                        let elt_sym = &string[element_start..element_end];
                        let elt = &PERIODIC_TABLE[elt_sym];
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: isotope
                        };
                        acc.inc(elt_spec, count);
                        element_start = 0;
                        element_end = 0;
                        isotope_start = 0;
                        isotope_end = 0;
                        count_start = 0;
                        count_end = 0;
                        if c == '(' {
                            paren_stack = 1;
                            group_start = i;
                            state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            element_start = i;
                            state = FormulaParserState::Element;
                        } else {
                            return Err("Invalid character found following element");
                        }
                    }
                },
                FormulaParserState::IsotopeToCount => {
                    if c.is_numeric() {
                        count_start = i;
                        state = FormulaParserState::Count;
                    } else {
                        let elt_sym = &string[element_start..element_end];
                        let elt = &PERIODIC_TABLE[elt_sym];
                        let isotope: u16 = match &string[isotope_start..isotope_end].parse::<u16>() {
                            Ok(val) => {*val},
                            Err(msg) => {return Err("Failed to parse integer from isotope count");}
                        };
                        let elt_spec = ElementSpecification {
                            element: elt,
                            isotope: isotope
                        };
                        acc.inc(elt_spec, 1);
                        element_start = 0;
                        element_end = 0;
                        isotope_start = 0;
                        isotope_end = 0;

                        if c == '(' {
                            paren_stack += 1;
                            group_start = i;
                            state = FormulaParserState::Group;
                        } else if c.is_ascii_uppercase() {
                            element_start = i;
                            state = FormulaParserState::Element;
                        } else {
                            return Err("Invalid character following isotope");
                        }
                    }
                },
                FormulaParserState::GroupToGroupCount => {
                    if !c.is_numeric() {
                        let group = match ChemicalComposition::parse(&string[group_end..group_end]) {
                            Ok(grp) => {grp},
                            Err(err) => {return Err(err)}
                        };
                        group_start = 0;
                        group_end = 0;
                        acc += &group;
                        if c == '(' {
                            paren_stack = 1;
                            group_start = i;
                            state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            element_start = i;
                            state = FormulaParserState::Element;
                        } else {
                            return Err("Invalid character found following element");
                        }
                    } else {
                        group_count_start = i;
                        state = FormulaParserState::GroupCount;
                    }
                }
                FormulaParserState::GroupCount => {
                    if !c.is_numeric() {
                        group_count_end = i;
                        let group = match ChemicalComposition::parse(&string[group_end..group_end]) {
                            Ok(grp) => {grp},
                            Err(err) => {return Err(err)}
                        };
                        group_start = 0;
                        group_end = 0;

                        let group_count: i32 = match &string[group_count_start..group_count_end].parse::<i32>() {
                            Ok(val) => {*val},
                            Err(msg) => {return Err("Failed to parse integer from element count");}
                        };
                        acc += &(&group * group_count);
                        group_count_start = 0;
                        group_count_end = 0;
                        if c == '(' {
                            paren_stack = 1;
                            group_start = i;
                            state = FormulaParserState::Group;
                        } else if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
                            element_start = i;
                            state = FormulaParserState::Element;
                        } else {
                            return Err("Invalid character found following element");
                        }
                    }
                }
            }
        }

        let i = n;
        match state {
            FormulaParserState::Element => {
                element_end = i;
                let elt_sym = &string[element_start..element_end];
                let elt = &PERIODIC_TABLE[elt_sym];
                let elt_spec = ElementSpecification {
                    element: elt,
                    isotope: 0
                };
                acc.inc(elt_spec, 1);
            },
            FormulaParserState::Count => {
                count_end = i;
                let count_parse = &string[count_start..count_end].parse::<i32>();
                let count: i32 = match count_parse {
                    Ok(val) => {*val},
                    Err(msg) => {return Err("Failed to parse integer from element count");}
                };
                let isotope: u16 = if isotope_end != isotope_start {
                    match &string[isotope_start..isotope_end].parse::<u16>() {
                        Ok(val) => {*val},
                        Err(msg) => {return Err("Failed to parse integer from isotope count");}
                    }
                } else {
                    0
                };
                let elt_sym = &string[element_start..element_end];
                let elt = &PERIODIC_TABLE[elt_sym];
                let elt_spec = ElementSpecification {
                    element: elt,
                    isotope: isotope
                };
                acc.inc(elt_spec, count);
            },
            FormulaParserState::GroupToGroupCount => {
                let group = match ChemicalComposition::parse(&string[group_end..group_end]) {
                    Ok(grp) => {grp},
                    Err(err) => {return Err(err)}
                };
                acc += &group;
            }
            FormulaParserState::GroupCount => {
                group_count_end = i;
                let group = match ChemicalComposition::parse(&string[group_end..group_end]) {
                    Ok(grp) => {grp},
                    Err(err) => {return Err(err)}
                };
                group_start = 0;
                group_end = 0;

                let group_count: i32 = match &string[group_count_start..group_count_end].parse::<i32>() {
                    Ok(val) => {*val},
                    Err(msg) => {return Err("Failed to parse integer from element count");}
                };
                acc += &(&group * group_count);
            }
            _ => {
                return Err("Incomplete formula")
            }
        }
        return Ok(acc);
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
mod test_chemical_composition {
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
        // Need to fix group parser
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
