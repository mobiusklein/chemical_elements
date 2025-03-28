//! Generate fine-grained isotopic patterns using a simple convolutional algorithm.
//!
//! Isotopic fine structure may

use std::mem::swap;

use super::{Peak, PeakList, TheoreticalIsotopicPattern};
use crate::{mass_charge_ratio, ChemicalComposition};

fn convolve_with(
    dist: &[(f64, f64)],
    element: &[(f64, f64)],
    out: &mut Vec<(f64, f64)>,
    abundance_threshold: f64,
) {
    for (iso_mass, iso_abundance) in element.iter().copied() {
        for (mz, inten) in dist.iter().copied() {
            let abundance = inten * iso_abundance;
            if abundance < abundance_threshold {
                continue;
            }
            out.push((mz + iso_mass, abundance))
        }
    }
}

fn convolve_pow(
    dist: &[(f64, f64)],
    n: i32,
    out: &mut Vec<(f64, f64)>,
    abundance_threshold: f64,
) {
    if n == 0 {
        out.push((0.0, 1.0))
    } else if n == 1 {
        out.extend_from_slice(dist);
    } else {
        let mut power = 2;
        let mut buffer: Vec<_> = Vec::from(dist);

        while power <= n {
            convolve_with(&buffer, &buffer, out, abundance_threshold);
            swap(&mut buffer, out);
            out.clear();
            power *= 2;
        }

        if power / 2 < n {
            let mut out2 = Vec::with_capacity(buffer.len() / 2);
            convolve_pow(dist, n - power / 2, &mut out2, abundance_threshold);
            convolve_with(&buffer, &out2, out, abundance_threshold);
        } else {
            swap(&mut buffer, out);
        }
    }
}


/// Generate a fine-grained isotopic pattern from a [`ChemicalComposition`]
/// with the specified charge state.
///
/// # Parameters
///
/// - `composition`: The chemical composition to compute the isotopic pattern for.
/// - `charge`: The charge state to compute the isotopic pattern in.
/// - `charge_carrier`: The mass shift of the charge carrier, e.g. the mass of a proton.
/// - `abundance_threshold`: The minimum abundance of an isotopologue to consider for inclusion.
///                          This applies to both the intermediate convolutions as well as the
///                          final peak list after normalization.
/// # Notes
///
/// This method will generate isotopic fine structure, which means that
/// it will contain many, many low abundance peaks corresponding to isotopomers
/// of each isotopologue.
/// ![An isotopic fine structure diagram][fine_structure]
#[cfg_attr(feature = "doc-only", cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("fine_structure", "doc/img/fine_structure.png")))]
#[cfg_attr(
not(feature = "doc-only"),
doc = "**Doc only not enabled**. Compile with feature `doc-only` and Rust version >= 1.54 \
           to enable."
)]
pub fn isotopic_convolution<'a, C: Into<ChemicalComposition<'a>>>(
    composition: C,
    charge: i32,
    charge_carrier: f64,
    abundance_threshold: f64,
) -> PeakList {
    let composition: ChemicalComposition<'a> = composition.into();
    let mut buffer = Vec::new();
    let mut out = Vec::new();
    let mut tmp = Vec::new();
    let mut tmp2 = Vec::new();
    for (i, (elt, count)) in composition.iter().enumerate() {
        buffer.extend(elt.element.isotopes.values().map(|i| (i.mass, i.abundance)));
        convolve_pow(&buffer, *count, &mut tmp, abundance_threshold);
        if i == 0 {
            swap(&mut tmp, &mut out);
        } else {
            convolve_with(&tmp, &out, &mut tmp2, abundance_threshold);
            swap(&mut out, &mut tmp2);
        }
        tmp.clear();
        tmp2.clear();
        buffer.clear();
    }
    out.sort_by(|a, b| a.0.total_cmp(&b.0));
    let peaks: Vec<_> = out.into_iter()
        .map(|(mass, intensity)| Peak {
            mz: if charge != 0 {
                mass_charge_ratio(mass, charge, charge_carrier)
            } else {
                mass
            },
            intensity,
        })
        .collect();

    let peaks = TheoreticalIsotopicPattern::from(peaks);
    peaks.normalize().ignore_below(abundance_threshold).peaks
}

#[cfg(test)]
mod test {
    use crate::PROTON;

    use super::*;

    #[test]
    fn test_base() {
        let comp = ChemicalComposition::parse("C3O4").unwrap();
        let peaks = isotopic_convolution(comp, 0, PROTON, 0.001);
        dbg!(&peaks);
    }
}
