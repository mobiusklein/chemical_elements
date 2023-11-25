//! Implementations of isotopic pattern generator algorithms.

pub mod baffling;
pub mod peak;
pub mod poisson;

pub use crate::isotopic_pattern::baffling::{
    isotopic_variants, BafflingRecursiveIsotopicPatternGenerator,
};
pub use crate::isotopic_pattern::peak::{Peak, PeakList, TheoreticalIsotopicPattern};
pub use crate::isotopic_pattern::poisson::{poisson_approximation, poisson_approximate_n_peaks_of};