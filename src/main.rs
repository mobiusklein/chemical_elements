use chemical_elements::{ChemicalComposition, ElementSpecification};
use std::convert::TryFrom;

fn main() {
    let spec = ElementSpecification::try_from("C[13]").unwrap();
    println!("Spec: {}", spec);

    let mut comp = ChemicalComposition::new();
    comp.set(ElementSpecification::try_from("C").unwrap(), 6);
    comp.set(ElementSpecification::try_from("H").unwrap(), 12);
    comp.set(ElementSpecification::try_from("O").unwrap(), 6);

    println!("Composition: {}, Mass: {}", comp.to_string(), comp.mass());
    println!(
        "Composition: {}, Mass: {}",
        ((&comp) * 2).to_string(),
        (&comp * 2).mass()
    );
    let _c1 = (&comp) + (&comp);

    println!(
        "formula no groups {}",
        ChemicalComposition::parse("C6H12O6")
            .expect("Parse without groups")
            .to_string()
    );
    println!(
        "formula one group {}",
        ChemicalComposition::parse("C6(H12)O6")
            .expect("Parse without groups")
            .to_string()
    );
}
