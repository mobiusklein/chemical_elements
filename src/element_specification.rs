use std::borrow::Borrow;
use std::cmp;
use std::fmt::{self, Display};
use std::hash;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

use crate::element::{Element, PeriodicTable};
use crate::table::PERIODIC_TABLE;

#[derive(Debug, Clone, Copy)]
pub enum ElementSpecificationParsingError {
    UnclosedIsotope,
    UnknownElement,
}

impl Display for ElementSpecificationParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ElementSpecificationParsingError {}

/// Classify a string as being an element specification
pub(crate) enum ElementSpecificationLike {
    /// Definitely an element specification, does not have an isotope
    Yes,
    /// Definitely not an element specification-like string
    No,
    /// Could be an element specification, looks element-like with an isotope
    Maybe,
}

impl From<bool> for ElementSpecificationLike {
    fn from(x: bool) -> Self {
        if x {
            ElementSpecificationLike::Yes
        } else {
            ElementSpecificationLike::No
        }
    }
}

#[cfg(feature="serde")]
mod serialize_element_ref {
    use serde::{Deserializer, Serializer, Deserialize};

    use crate::Element;
    use crate::table::PERIODIC_TABLE;

    pub fn serialize<S>(val: &&Element, serializer: S) -> Result<S::Ok, S::Error>  where S: Serializer {
        serializer.serialize_str(&val.symbol)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<&'static Element, D::Error> where D: Deserializer<'de> {
        match String::deserialize(deserializer) {
            Ok(symbol) => {
                Ok(&PERIODIC_TABLE[&symbol])
            },
            Err(err) => {
                Err(err)
            }
        }

    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
/// A hashable key referencing an element with a specific isotope
/// state. `element` is the [`Element`](crate::Element) represented, and `isotope` is
/// the isotope number, though 0 means monoisotopic.
///
/// Meant to be used as the keys for [`ChemicalComposition`]
pub struct ElementSpecification<'element> {
    #[cfg_attr(feature="serde", serde(with="serialize_element_ref"))]
    pub element: &'element Element,
    pub isotope: u16,
}

impl<'a> cmp::PartialEq for ElementSpecification<'a> {
    #[inline]
    fn eq(&self, other: &ElementSpecification) -> bool {
        if self.element != other.element {
            return false;
        }
        return self.isotope == other.isotope;
    }
}

impl<'a> cmp::PartialEq<str> for ElementSpecification<'a> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.element.symbol == other && self.isotope == 0
    }
}

impl<'a> cmp::Eq for ElementSpecification<'a> {}

impl<'element> hash::Hash for ElementSpecification<'element> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl<'a> Borrow<str> for ElementSpecification<'a> {
    fn borrow(&self) -> &str {
        &self.element.symbol
    }
}

impl<'element> fmt::Display for ElementSpecification<'element> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.isotope == 0 {
            f.write_str(&self.element.symbol)
        } else {
            write!(f, "{}[{}]", self.element.symbol, self.isotope)
        }
    }
}

impl<'transient, 'lifespan: 'transient, 'element> ElementSpecification<'element> {
    pub fn new(element: &'element Element, isotope: u16) -> ElementSpecification<'element> {
        return ElementSpecification { element, isotope };
    }

    #[inline]
    pub fn to_string(&self) -> String {
        if self.isotope == 0 {
            return format!("{}", self.element.symbol);
        } else {
            return format!("{}[{}]", self.element.symbol, self.isotope);
        }
    }

    #[inline]
    pub fn parse(
        string: &'transient str,
    ) -> Result<ElementSpecification<'lifespan>, ElementSpecificationParsingError> {
        Self::parse_with(string, &PERIODIC_TABLE)
    }

    pub(crate) fn quick_check_str(string: &str) -> ElementSpecificationLike {
        let n = string.len();
        let mut chars = string.chars();
        if n == 0 {
            return ElementSpecificationLike::No
        } else if n == 1 {
            let first = chars.nth(0).unwrap();
            (first.is_alphabetic()).into()
        }
        // The one or two letter scenario, most common
        else if n < 3 {
            let first = chars.nth(0).unwrap();
            let last = chars.last().unwrap();
            (last != '[' && last != ']' && first.is_alphabetic()).into()
        } else if n == 4 {
            let first = chars.nth(0).unwrap();
            let last = chars.last().unwrap();
            if first.is_alphabetic() {
                if last == ']' {
                    ElementSpecificationLike::Maybe
                } else {
                    ElementSpecificationLike::No
                }
            } else {
                ElementSpecificationLike::No
            }
        } else {
            ElementSpecificationLike::Maybe
        }
    }

    #[inline]
    pub fn parse_with(
        string: &'transient str,
        periodic_table: &'lifespan PeriodicTable,
    ) -> Result<ElementSpecification<'lifespan>, ElementSpecificationParsingError> {
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
                    return Err(ElementSpecificationParsingError::UnclosedIsotope);
                }
            } else if c == ']' {
                iso_end = i;
            }
        }
        let elt_sym = &string[elt_start..elt_end];
        if let Some(element) = periodic_table.get(elt_sym) {
            let isotope = if iso_start != iso_end {
                string[iso_start..iso_end].parse::<u16>().unwrap()
            } else {
                0
            };
            Ok(ElementSpecification::new(element, isotope))
        } else {
            Err(ElementSpecificationParsingError::UnknownElement)
        }
    }
}


impl<'a> FromStr for ElementSpecification<'a> {
    type Err = ElementSpecificationParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match ElementSpecification::parse(s) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        };
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_element_spec_parse() {
        let spec = ("C[13]").parse::<ElementSpecification>().unwrap();
        assert_eq!(spec.isotope, 13);
        assert_eq!(spec.element.symbol, "C");
    }
}