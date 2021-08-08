//! A library for representing chemical compositions, managing
//! elemental formulae and generating (coarse) isotopic patterns.
//!
//! ## Chemical Compositions
//! ```
//! use chemical_elements::{ChemicalComposition, ElementSpecification};
//!
//! let mut composition = ChemicalComposition::parse("H2O").unwrap();
//! composition["C"] = 6;
//! composition["O"] = 6;
//! composition["H"] = 12;
//! assert!((composition.mass() - 180.06339).abs() < 1e-6)
//! ```
mod composition;
mod element;
mod formula;
mod helper;
pub mod isotopic_pattern;
mod mz;
mod table;

pub use crate::composition::{
    ChemicalComposition, ElementSpecification, ElementSpecificationParsingError,
};
pub use crate::element::{Element, Isotope, PeriodicTable};
pub use crate::formula::{parse_formula, parse_formula_with_table, FormulaParserError};
pub use crate::mz::{mass_charge_ratio, neutral_mass, PROTON};
pub use crate::table::PERIODIC_TABLE;
pub use helper::ChemicalElements;
