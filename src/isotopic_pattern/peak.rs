use std::cmp;
use std::fmt;
use std::ops;

use crate::mz::{neutral_mass, PROTON};

#[derive(Debug, Clone, Default)]
/**A theoretical peak for an isotopic pattern */
pub struct Peak {
    /// The m/z of the isotopic peak
    pub mz: f64,
    /// The theoretical abundance of the isotopic peak, usually expressed as a
    /// percentage of total signal, unless scaled.
    pub intensity: f64,
    /// The charge state of the isotopic peak
    pub charge: i32,
}

impl fmt::Display for Peak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peak({}, {}, {})", self.mz, self.intensity, self.charge)
    }
}

impl cmp::PartialEq<Peak> for Peak {
    #[inline]
    fn eq(&self, other: &Peak) -> bool {
        if (self.mz - other.mz).abs() > 1e-3 {
            return false;
        } else if (self.intensity - other.intensity).abs() > 1e-3 {
            return false;
        }
        return true;
    }
}

impl Eq for Peak {}

impl cmp::PartialOrd<Peak> for Peak {
    #[inline]
    fn partial_cmp(&self, other: &Peak) -> Option<cmp::Ordering> {
        return self.mz.partial_cmp(&other.mz);
    }
}

impl Peak {
    #[inline]
    pub fn mz(&self) -> f64 {
        self.mz
    }

    #[inline]
    pub fn intensity(&self) -> f32 {
        self.intensity as f32
    }

    #[inline]
    pub fn charge(&self) -> i32 {
        self.charge
    }

    #[inline]
    pub fn neutral_mass(&self) -> f64 {
        if self.charge == 0 {
            self.mz
        } else {
            neutral_mass(self.mz, self.charge, PROTON)
        }
    }
}

#[cfg(feature = "mzpeaks")]
mod mzpeaks_interface {
    use super::*;
    use mzpeaks;

    impl mzpeaks::CoordinateLike<mzpeaks::MZ> for Peak {
        #[inline]
        fn coordinate(&self) -> f64 {
            self.mz
        }
    }

    impl mzpeaks::IntensityMeasurement for Peak {
        #[inline]
        fn intensity(&self) -> f32 {
            self.intensity()
        }
    }

    impl mzpeaks::IndexedCoordinate<mzpeaks::MZ> for Peak {
        fn get_index(&self) -> mzpeaks::IndexType {
            0
        }

        fn set_index(&mut self, _index: mzpeaks::IndexType) {}
    }
}

pub type PeakList = Vec<Peak>;

#[derive(Debug, Clone)]
/**
A theoretical isotopic pattern that supports a variety of mutating
transformations.
*/
pub struct TheoreticalIsotopicPattern {
    pub peaks: PeakList,
    pub origin: f64,
}

impl TheoreticalIsotopicPattern {
    #[inline]
    pub fn new(peaks: PeakList, origin: f64) -> TheoreticalIsotopicPattern {
        TheoreticalIsotopicPattern { peaks, origin }
    }

    #[inline]
    pub fn len(&self) -> usize {
        return self.peaks.len();
    }

    #[inline]
    /**
    Clone this peak list, omitting the last peak and normalizing it
    along the way.
    */
    pub fn clone_drop_last(&self) -> TheoreticalIsotopicPattern {
        let n = self.len();
        let mut peaks = PeakList::with_capacity(n);
        for (i, peak) in self.peaks.iter().enumerate() {
            if i == n {
                break;
            }
            peaks.push(peak.clone());
        }

        let result = TheoreticalIsotopicPattern {
            peaks,
            origin: self.origin,
        };
        result.normalize()
    }

    #[inline]
    /**
    Shift the m/z of each peak in the list by `offset`
    */
    pub fn shift(mut self, offset: f64) -> TheoreticalIsotopicPattern {
        self.origin += offset;
        for peak in &mut self {
            peak.mz += offset;
        }
        self
    }

    #[inline]
    /**Clone the peak list, shifting the m/z of the generated peaks by `offset`
    along the way*/
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

    #[inline]
    /**Compute the sum of the intensities for this peak list*/
    pub fn total(&self) -> f64 {
        self.peaks.iter().map(|p| p.intensity).sum()
    }

    #[inline]
    /**Scale the intensity of each peak by `factor` */
    pub fn scale_by(mut self, factor: f64) -> TheoreticalIsotopicPattern {
        for p in self.peaks.iter_mut() {
            p.intensity *= factor;
        }
        return self;
    }

    #[inline]
    /**Normalize the intensity of each peak in the isotopic pattern such that the total
    sums to 1.0*/
    pub fn normalize(self) -> TheoreticalIsotopicPattern {
        let total = self.total();
        self.scale_by(1.0 / total)
    }

    #[inline]
    /**Truncate the peak list after the cumulative intensity meets or exceeds `threshold` */
    pub fn truncate_after(mut self, threshold: f64) -> TheoreticalIsotopicPattern {
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

    #[inline]
    /**Drop any peaks in the isotopic pattern whose intensity is below `threshold` */
    pub fn ignore_below(mut self, threshold: f64) -> TheoreticalIsotopicPattern {
        let mut acc = PeakList::with_capacity(self.len());
        for peak in self.peaks.drain(..) {
            if peak.intensity >= threshold {
                acc.push(peak);
            }
        }
        self.peaks = acc;
        self.normalize()
    }

    #[inline]
    pub fn iter(&self) -> TheoreticalIsotopicPatternIter {
        TheoreticalIsotopicPatternIter::new(self)
    }

    #[inline]
    pub fn iter_mut(&mut self) -> TheoreticalIsotopicPatternIterMut {
        TheoreticalIsotopicPatternIterMut::new(self)
    }
}

impl fmt::Display for TheoreticalIsotopicPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "TheoreticalIsotopicPattern([")?;
        let n = self.len() - 1;
        for (i, peak) in self.into_iter().enumerate() {
            write!(f, "{}", peak)?;
            if i != n {
                write!(f, ", ")?;
            }
        }
        write!(f, "])")?;
        return Ok(());
    }
}

impl ops::Index<usize> for TheoreticalIsotopicPattern {
    type Output = Peak;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        return &(self.peaks[i]);
    }
}

impl<'a> IntoIterator for &'a TheoreticalIsotopicPattern {
    type Item = &'a Peak;
    type IntoIter = TheoreticalIsotopicPatternIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl<'a> IntoIterator for &'a mut TheoreticalIsotopicPattern {
    type Item = &'a mut Peak;
    type IntoIter = TheoreticalIsotopicPatternIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self)
    }
}

impl From<PeakList> for TheoreticalIsotopicPattern {
    #[inline]
    fn from(src: PeakList) -> Self {
        let origin = src[0].mz;
        Self::new(src, origin)
    }
}

impl PartialEq for TheoreticalIsotopicPattern {
    fn eq(&self, other: &Self) -> bool {
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl PartialEq<[Peak]> for TheoreticalIsotopicPattern {
    fn eq(&self, other: &[Peak]) -> bool {
        for (a, b) in self.iter().zip(other.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

impl Eq for TheoreticalIsotopicPattern {}

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

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}

pub struct TheoreticalIsotopicPatternIterMut<'a> {
    iter: std::slice::IterMut<'a, Peak>,
}

impl<'a> TheoreticalIsotopicPatternIterMut<'a> {
    fn new(peaks: &'a mut TheoreticalIsotopicPattern) -> TheoreticalIsotopicPatternIterMut<'a> {
        return TheoreticalIsotopicPatternIterMut {
            iter: peaks.peaks.iter_mut(),
        };
    }
}

impl<'a> Iterator for TheoreticalIsotopicPatternIterMut<'a> {
    type Item = &'a mut Peak;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        return self.iter.next();
    }
}
