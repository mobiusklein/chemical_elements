use std::cmp;
use std::fmt;
use std::ops;

use crate::mz::{neutral_mass, PROTON};

#[derive(Debug, Clone, Default)]
pub struct Peak {
    pub mz: f64,
    pub intensity: f64,
    pub charge: i32,
}

impl fmt::Display for Peak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peak({}, {}, {})", self.mz, self.intensity, self.charge)
    }
}

impl cmp::PartialEq<Peak> for Peak {
    fn eq(&self, other: &Peak) -> bool {
        if (self.mz - other.mz).abs() > 1e-3 {
            return false;
        } else if (self.intensity - other.intensity).abs() > 1e-3 {
            return false;
        }
        return true;
    }
}

impl cmp::PartialOrd<Peak> for Peak {
    fn partial_cmp(&self, other: &Peak) -> Option<cmp::Ordering> {
        return self.mz.partial_cmp(&other.mz);
    }
}

impl Peak {
    pub fn mzs(&self) -> f64 {
        self.mz
    }

    pub fn intensity(&self) -> f32 {
        self.intensity as f32
    }

    pub fn charge(&self) -> i32 {
        self.charge
    }

    pub fn neutral_mass(&self) -> f64 {
        neutral_mass(self.mz, self.charge, PROTON)
    }
}

pub type PeakList = Vec<Peak>;

#[derive(Debug, Clone)]
pub struct TheoreticalIsotopicPattern {
    pub peaks: PeakList,
    pub origin: f64,
}

impl TheoreticalIsotopicPattern {
    pub fn new(peaks: PeakList, origin: f64) -> TheoreticalIsotopicPattern {
        TheoreticalIsotopicPattern { peaks, origin }
    }

    pub fn len(&self) -> usize {
        return self.peaks.len();
    }

    pub fn clone_drop_last(&self) -> TheoreticalIsotopicPattern {
        let n = self.len();
        let mut peaks = PeakList::with_capacity(n);
        for (i, peak) in self.peaks.iter().enumerate() {
            if i == n {
                break;
            }
            peaks.push(peak.clone());
        }

        let mut result = TheoreticalIsotopicPattern {
            peaks,
            origin: self.origin,
        };
        result.normalize();
        result
    }

    pub fn clone_shifted(&self, offset: f64) -> TheoreticalIsotopicPattern {
        let n = self.len();
        let mut peaks = PeakList::with_capacity(n);
        for peak in self {
            let mut shifted = peak.clone();
            shifted.mz += offset;
            peaks.push(shifted);
        }
        TheoreticalIsotopicPattern::new(peaks, self.origin + offset)
    }

    pub fn total(&self) -> f64 {
        self.peaks.iter().map(|p| p.intensity).sum()
    }

    pub fn scale_by(&mut self, factor: f64) -> &TheoreticalIsotopicPattern {
        for p in self.peaks.iter_mut() {
            p.intensity *= factor;
        }
        return self;
    }

    pub fn normalize(&mut self) -> &TheoreticalIsotopicPattern {
        let total = self.total();
        self.scale_by(1.0 / total)
    }

    pub fn truncate_after(&mut self, threshold: f64) -> &TheoreticalIsotopicPattern {
        let mut total = 0.0;
        let mut stop_index = 0;
        for (i, p) in self.peaks.iter().enumerate() {
            total += p.intensity;
            if total >= threshold {
                stop_index = i;
                break;
            }
        }
        self.peaks.truncate(stop_index);
        self.normalize()
    }

    pub fn ignore_below(&mut self, threshold: f64) -> &TheoreticalIsotopicPattern {
        let mut acc = PeakList::with_capacity(self.len());
        for peak in self.peaks.drain(..) {
            if peak.intensity >= threshold {
                acc.push(peak);
            }
        }
        self.peaks = acc;
        self.normalize()
    }
}

impl fmt::Display for TheoreticalIsotopicPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TheoreticalIsotopicPattern([").expect("Write failed");
        let n = self.len() - 1;
        for (i, peak) in self.into_iter().enumerate() {
            write!(f, "{}", peak).expect("Write failed");
            if i != n {
                write!(f, ", ").expect("Write failed");
            }
        }
        write!(f, "])").expect("Write failed");
        return Ok(());
    }
}

impl ops::Index<usize> for TheoreticalIsotopicPattern {
    type Output = Peak;

    fn index(&self, i: usize) -> &Self::Output {
        return &(self.peaks[i]);
    }
}

impl<'a> IntoIterator for &'a TheoreticalIsotopicPattern {
    type Item = &'a Peak;
    type IntoIter = TheoreticalIsotopicPatternIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        return Self::IntoIter::new(self);
    }
}

impl From<PeakList> for TheoreticalIsotopicPattern {
    fn from(src: PeakList) -> Self {
        let origin = src[0].mz;
        Self::new(src, origin)
    }
}

// Iterators

pub struct TheoreticalIsotopicPatternIter<'a> {
    iter: std::slice::Iter<'a, Peak>,
}

impl<'a> TheoreticalIsotopicPatternIter<'a> {
    fn new(peaks: &'a TheoreticalIsotopicPattern) -> TheoreticalIsotopicPatternIter<'a> {
        return TheoreticalIsotopicPatternIter {
            iter: peaks.peaks.iter(),
        };
    }
}

impl<'a> Iterator for TheoreticalIsotopicPatternIter<'a> {
    type Item = &'a Peak;

    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}
