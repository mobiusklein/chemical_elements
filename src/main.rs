use chemical_elements::{ChemicalComposition, PROTON};
use chemical_elements::isotopic_pattern::isotopic_variants;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let buffer = args.next().expect("Expected a chemical formula.");
    println!("Read {}", buffer);
    let comp = ChemicalComposition::parse(&buffer).expect("Failed to parse chemical formula");
    let dist = isotopic_variants(comp, 0, 0, PROTON);
    for p in dist {
        println!("{}\t{}", p.neutral_mass(), p.intensity());
    }

}
