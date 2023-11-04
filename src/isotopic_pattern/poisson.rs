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
    for i in 0..n_peaks {
        let mz = mass_charge_ratio(mass + (i as f64 * NEUTRON_SHIFT), charge, PROTON);
        let peak = Peak {
            mz,
            intensity: intensities[i] / total,
            charge: charge,
        };
        peak_list.push(peak);
    }

    peak_list
}


pub fn poisson_approximation(
    mass: f64,
    n_peaks: usize,
    charge: i32,
) -> PeakList {
    poisson_approximation_impl(mass, n_peaks, charge, LAMBDA_FACTOR)
}


#[cfg(test)]
mod test {
    use crate::isotopic_pattern::poisson::NEUTRON_SHIFT;

    use super::poisson_approximation;
    use super::super::{Peak, PeakList};

    #[test]
    fn test_approximate() {
        let peak_list: PeakList = poisson_approximation(750.0, 4, 2);
        assert_eq!(peak_list.len(), 4);

        let mut acc = 0.0;
        for i in 0..peak_list.len() {
            let peak: &Peak = &peak_list[i];
            acc += peak.intensity;
            let mz_delta = peak.mz - 376.007276;
            if mz_delta < 0.0 {
                assert!(mz_delta > -1e-3);
            } else {
                assert!(mz_delta < NEUTRON_SHIFT * 4.0);
            }
        }
        assert!((acc - 1.0).abs() < 1e-3);
    }
}