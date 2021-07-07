use std::convert::TryFrom;
use chemical_elements::{ElementSpecification, ChemicalComposition};

fn main() {
    let spec = ElementSpecification::try_from("C[13]").unwrap();
    println!("Spec: {}", spec);

    let mut comp = ChemicalComposition::new();
    comp.set(ElementSpecification::try_from("C").unwrap(), 6);
    comp.set(ElementSpecification::try_from("H").unwrap(), 12);
    comp.set(ElementSpecification::try_from("O").unwrap(), 6);

    println!("Composition: {}, Mass: {}", comp.to_string(), comp.mass());
    println!("Composition: {}, Mass: {}", ((&comp) * 2).to_string(), (&comp * 2).mass());
    let c1 = (&comp) + (&comp);

    for (k, _v) in c1.iter() {
        println!("{} => {}", k.element, k.element as *const _ as usize);
    }
    for (k, _v) in comp.iter() {
        println!("{} => {}", k.element, k.element as *const _ as usize);
    }
}