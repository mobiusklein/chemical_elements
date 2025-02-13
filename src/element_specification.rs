use std::borrow::Borrow;
use std::cmp;
use std::fmt::{self, Display};
use std::hash;
use std::str::FromStr;

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

#[derive(Debug, Clone, Copy)]
#[cfg_attr(
    feature = "serde",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr)
)]
/// A hashable key referencing an element with a specific isotope
/// state. `element` is the [`Element`](crate::Element) represented, and `isotope` is
/// the isotope number, though 0 means monoisotopic.
///
/// Meant to be used as the keys for [`ChemicalCompositionLike`](crate::ChemicalCompositionLike)
pub struct ElementSpecification<'element> {
    pub element: &'element Element,
    pub isotope: u16,
}

impl cmp::PartialEq for ElementSpecification<'_> {
    #[inline]
    fn eq(&self, other: &ElementSpecification) -> bool {
        if self.element != other.element {
            return false;
        }
        self.isotope == other.isotope
    }
}

impl cmp::PartialEq<str> for ElementSpecification<'_> {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.element.symbol == other && self.isotope == 0
    }
}

impl cmp::Eq for ElementSpecification<'_> {}

impl hash::Hash for ElementSpecification<'_> {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.element.hash(state);
    }
}

impl Borrow<str> for ElementSpecification<'_> {
    fn borrow(&self) -> &str {
        &self.element.symbol
    }
}

impl fmt::Display for ElementSpecification<'_> {
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
        ElementSpecification { element, isotope }
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
            ElementSpecificationLike::No
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

impl FromStr for ElementSpecification<'_> {
    type Err = ElementSpecificationParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match ElementSpecification::parse(s) {
            Ok(r) => Ok(r),
            Err(err) => Err(err),
        }
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
