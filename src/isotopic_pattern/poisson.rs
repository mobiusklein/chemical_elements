//! Approximate the maximum number of isotopic peaks to include in an isotopic distribution
//! approximation for biomolecules using the Poisson distribution, using the method described
//! in Bellew et al:
//!
//! Bellew, M., Coram, M., Fitzgibbon, M., Igra, M., Randolph, T., Wang, P., May, D., Eng, J., Fang, R., Lin, C., Chen, J.,
//! Goodlett, D., Whiteaker, J., Paulovich, A., & Mcintosh, M. (2006). A suite of algorithms for the comprehensive analysis
//! of complex protein mixtures using high-resolution LC-MS. 22(15), 1902–1909. <https://doi.org/10.1093/bioinformatics/btl276>

use super::{Peak, PeakList};
use crate::mz::{mass_charge_ratio, PROTON};

const NEUTRON_SHIFT: f64 = 1.0033548378;
const LAMBDA_FACTOR: f64 = 1800.0;

pub fn poisson_approximation_impl(
    mass: f64,
    n_peaks: usize,
    charge: i32,
    lambda_factor: f64,
) -> PeakList {
    let mut peak_list = PeakList::new();
    if n_peaks == 0 {
        return peak_list;
    }
    let lambda = mass / lambda_factor;
    let mut p_i = 1.0;
    let mut factorial_acc = 1.0;
    let mut total = 1.0;

    let mut intensities = Vec::with_capacity(n_peaks);
    intensities.push(1.0);
    for i in 1..n_peaks {
        p_i *= lambda;
        factorial_acc *= i as f64;
        let cur_intensity = p_i / factorial_acc;
        if cur_intensity.is_finite() {
            intensities.push(cur_intensity);
            total += cur_intensity;
        } else {
            intensities.push(0.0);
        }
    }
    (0..n_peaks).for_each(|i| {
        let mz = mass_charge_ratio(mass + (i as f64 * NEUTRON_SHIFT), charge, PROTON);
        let peak = Peak {
            mz,
            intensity: intensities[i] / total,
        };
        peak_list.push(peak);
    });

    peak_list
}

pub fn poisson_approximate_n_peaks_of_impl(
    mass: f64,
    lambda_factor: f64,
    threshold: f64,
    max_iter: usize,
) -> usize {
    let lambda = mass / lambda_factor;
    let mut p_i = 1.0;
    let mut factorial_acc = 1.0;
    let mut acc = 1.0;

    let target_threshold = 1.0 - threshold;

    for i in 1..max_iter {
        p_i *= lambda;
        factorial_acc *= i as f64;
        let cur_intensity = p_i / factorial_acc;
        if cur_intensity.is_infinite() {
            return i;
        }
        acc += cur_intensity;
        if cur_intensity / acc < target_threshold {
            return i;
        }
    }
    max_iter
}

/// This algorithm approximates the isotopic pattern of `mass` at `charge`
/// with `n_peaks` peaks included.
pub fn poisson_approximation(mass: f64, n_peaks: usize, charge: i32) -> PeakList {
    poisson_approximation_impl(mass, n_peaks, charge, LAMBDA_FACTOR)
}

/// This algorithm approximates the number of peaks in an isotopic pattern of `mass`
/// until `threshold`% signal is generated.
pub fn poisson_approximate_n_peaks_of(mass: f64, threshold: f64) -> usize {
    poisson_approximate_n_peaks_of_impl(mass, LAMBDA_FACTOR, threshold, 255)
}

#[cfg(test)]
mod test {
    use crate::isotopic_pattern::poisson::NEUTRON_SHIFT;

    use super::super::{Peak, PeakList};
    use super::{poisson_approximate_n_peaks_of, poisson_approximation};

    #[test]
    fn test_approximate() {
        let peak_list: PeakList = poisson_approximation(750.0, 4, 2);
        assert_eq!(peak_list.len(), 4);

        let mut acc = 0.0;
        (0..peak_list.len()).for_each(|i| {
            let peak: &Peak = &peak_list[i];
            acc += peak.intensity;
            let mz_delta = peak.mz - 376.007276;
            if mz_delta < 0.0 {
                assert!(mz_delta > -1e-3);
            } else {
                assert!(mz_delta < NEUTRON_SHIFT * 4.0);
            }
        });
        assert!((acc - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_approximate_n_peaks() {
        let n = poisson_approximate_n_peaks_of(750.0, 0.95);
        assert_eq!(n, 3);
    }

    #[test]
    fn test_approximate_n_peaks_zero() {
        let n = poisson_approximate_n_peaks_of(0.0, 0.95);
        eprintln!("{n}");
        assert!(n > 0, "{n} should not be zero!");
    }

    #[test]
    fn test_approximate_n_peaks_overflow() {
        let n = poisson_approximate_n_peaks_of(39999000.234256, 0.9999);
        assert!(n > 0, "{n} should not be zero!");
    }
}
