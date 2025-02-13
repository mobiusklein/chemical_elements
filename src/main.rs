use chemical_elements::isotopic_pattern::isotopic_variants;
use chemical_elements::{ChemicalComposition, PROTON};
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let buffer = args.next().expect("Expected a chemical formula.");
    println!("Read {}", buffer);
    let npeaks: Option<_> = args
        .next()
        .map(|val| val.parse::<i32>().expect("Failed to parse n_peaks"));
    let comp: ChemicalComposition = buffer.parse().expect("Failed to parse chemical formula");
    let dist = match npeaks {
        Some(n) => isotopic_variants(comp, n, 0, PROTON),
        None => isotopic_variants(comp, 0, 0, PROTON),
    };
    for p in dist {
        println!("{}\t{}", p.mz(), p.intensity());
    }
}
