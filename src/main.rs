use std::convert::TryFrom;
use chemical_elements::{ElementSpecification, ChemicalComposition, PROTON};
use chemical_elements::isotopic_pattern::{isotopic_variants};


fn main() {
    let spec = ElementSpecification::try_from("C[13]").unwrap();
    println!("Spec: {}", spec);

    let mut comp = ChemicalComposition::new();
    comp.set(ElementSpecification::try_from("C").unwrap(), 6);
    comp.set(ElementSpecification::try_from("H").unwrap(), 12);
    comp.set(ElementSpecification::try_from("O").unwrap(), 6);

    println!("Composition: {}, Mass: {}", comp.to_string(), comp.mass());
    println!("Composition: {}, Mass: {}", ((&comp) * 2).to_string(), (&comp * 2).mass());
    let _c1 = (&comp) + (&comp);
    let peaks = isotopic_variants(&comp, 6, 0, PROTON);
    for peak in peaks.iter() {
        println!("Peak: {}, {}, {}", peak.mz, peak.intensity, peak.charge);
    }
}