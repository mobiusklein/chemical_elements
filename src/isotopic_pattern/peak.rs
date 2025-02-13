use std::cmp;
use std::fmt;
use std::ops;
use std::ops::Range;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
/**A theoretical peak for an isotopic pattern */
pub struct Peak {
    /// The m/z of the isotopic peak
    pub mz: f64,
    /// The theoretical abundance of the isotopic peak, usually expressed as a
    /// percentage of total signal, unless scaled.
    pub intensity: f64,
}

impl fmt::Display for Peak {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Peak({}, {})", self.mz, self.intensity)
    }
}

impl cmp::PartialEq<Peak> for Peak {
    #[inline]
    fn eq(&self, other: &Peak) -> bool {
        if (self.mz - other.mz).abs() > 1e-3 || (self.intensity - other.intensity).abs() > 1e-3 {
            return false;
        }
        true
    }
}

impl Eq for Peak {}

impl cmp::PartialOrd<Peak> for Peak {
    #[inline]
    fn partial_cmp(&self, other: &Peak) -> Option<cmp::Ordering> {
        self.mz.partial_cmp(&other.mz)
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
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
        self.peaks.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.peaks.is_empty()
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
            peaks.push(*peak);
        }

        let result = TheoreticalIsotopicPattern {
            peaks,
            origin: self.origin,
        };
        result.normalize()
    }

    /**
    Copy a slice of this isotopic pattern and re-normalize it so that slice sums to 1.0
    */
    pub fn slice_normalized(&self, range: Range<usize>) -> Self {
        let slc = &self.peaks[range];
        let subset = Self::new(slc.to_vec(), self.origin);
        subset.normalize()
    }

    /// Create an iterator that yields successively right-truncated versions of ``self`` as long as those
    /// truncations cover at least ``threshold`` percent of the original isotopic pattern
    pub fn incremental_truncation(self, threshold: f64) -> IncrementalTruncationIter {
        IncrementalTruncationIter::new(self.normalize(), threshold)
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
            let mut shifted = *peak;
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
        self
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
        self.peaks.truncate(stop_index + 1);
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

    pub fn truncate_after_ignore_below_shift_normalize(
        mut self,
        truncate_threshold: f64,
        ignore_below_threshold: f64,
        shift: f64,
    ) -> Self {
        let mut total = 0.0;
        let mut stop_index = 0;
        for (i, p) in self.peaks.iter().enumerate() {
            total += p.intensity;
            if total >= truncate_threshold {
                stop_index = i;
                break;
            }
        }

        self.peaks.truncate(stop_index + 1);
        let ignore_below_threshold = ignore_below_threshold / total;
        let mut acc = PeakList::with_capacity(stop_index);
        for mut peak in self.peaks.into_iter() {
            if peak.intensity >= ignore_below_threshold {
                peak.mz += shift;
                acc.push(peak);
            } else {
                total -= peak.intensity;
            }
        }
        self.peaks = acc;

        for peak in self.peaks.iter_mut() {
            peak.intensity /= total;
        }

        self
    }

    #[inline]
    pub fn iter(&self) -> TheoreticalIsotopicPatternIter {
        self.peaks.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> TheoreticalIsotopicPatternIterMut {
        self.peaks.iter_mut()
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
        Ok(())
    }
}

impl ops::Index<usize> for TheoreticalIsotopicPattern {
    type Output = Peak;

    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &(self.peaks[i])
    }
}

impl IntoIterator for TheoreticalIsotopicPattern {
    type Item = Peak;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.peaks.into_iter()
    }
}

impl<'a> IntoIterator for &'a TheoreticalIsotopicPattern {
    type Item = &'a Peak;
    type IntoIter = TheoreticalIsotopicPatternIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.peaks.iter()
    }
}

impl<'a> IntoIterator for &'a mut TheoreticalIsotopicPattern {
    type Item = &'a mut Peak;
    type IntoIter = TheoreticalIsotopicPatternIterMut<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.peaks.iter_mut()
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

pub type TheoreticalIsotopicPatternIter<'a> = std::slice::Iter<'a, Peak>;
pub type TheoreticalIsotopicPatternIterMut<'a> = std::slice::IterMut<'a, Peak>;

/**
An [`Iterator`] that produces successively truncated versions of a [`TheoreticalIsotopicPattern`]
*/
pub struct IncrementalTruncationIter {
    pub threshold: f64,
    pub template: TheoreticalIsotopicPattern,
    index: usize,
    cumulative: Vec<f64>,
}

impl IncrementalTruncationIter {
    pub fn new(template: TheoreticalIsotopicPattern, threshold: f64) -> Self {
        let cumulative = template.iter().fold(
            Vec::with_capacity(template.len()),
            |mut state: Vec<f64>, p| {
                if state.is_empty() {
                    state.push(p.intensity);
                } else {
                    state.push(state.last().unwrap() + p.intensity);
                };
                state
            },
        );
        let index = template.len().saturating_sub(1);
        Self {
            template,
            threshold,
            index,
            cumulative,
        }
    }
}

impl Iterator for IncrementalTruncationIter {
    type Item = TheoreticalIsotopicPattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > 0 && self.cumulative[self.index] > self.threshold {
            let result = self.template.slice_normalized(0..self.index + 1);
            self.index = self.index.saturating_sub(1);
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::isotopic_pattern::poisson_approximation;

    fn make_tid() -> TheoreticalIsotopicPattern {
        TheoreticalIsotopicPattern::new(poisson_approximation(1200.0, 8, 2), 1200.0)
    }

    #[test]
    fn test_truncate_after() {
        let peaks = make_tid();
        let n = peaks.len();

        let peaks_trunc = peaks.clone().truncate_after(0.95);
        let nt = peaks_trunc.len();

        let trunc_frac: f64 = peaks.iter().take(nt).map(|p| p.intensity).sum();
        assert!(trunc_frac >= 0.95);
        let trunc_frac2: f64 = peaks.iter().take(nt - 1).map(|p| p.intensity).sum();
        assert!(trunc_frac2 <= 0.95);

        assert_eq!(n, 8);
        assert_eq!(nt, 3);
    }

    #[test]
    fn test_ignore_below() {
        let peaks = make_tid();
        let n = peaks.len();

        let peaks_trunc = peaks.clone().ignore_below(0.001);
        let nt = peaks_trunc.len();
        assert!(peaks.iter().skip(nt).all(|p| p.intensity <= 0.001));

        assert_eq!(n, 8);
        assert_eq!(nt, 5);
    }

    #[test]
    fn test_truncate_after_ignore_below() {
        let peaks = make_tid();

        let peaks_trunc = peaks
            .clone()
            .truncate_after_ignore_below_shift_normalize(0.95, 0.001, 0.0);
        let peaks_trunc2 = peaks.clone().truncate_after(0.95).ignore_below(0.001);

        assert_eq!(peaks_trunc, peaks_trunc2)
    }

    #[test]
    fn test_incremental_iter() {
        let peaks = make_tid();
        let forms: Vec<_> = peaks.clone().incremental_truncation(0.95).collect();
        assert!(forms.contains(&peaks));
        assert_eq!(forms.len(), 6);
        assert_eq!(forms.last().unwrap().len(), 3);
    }
}
