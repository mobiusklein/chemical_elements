use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chemical_elements::{ChemicalComposition, PROTON};
use chemical_elements::isotopic_pattern::BafflingRecursiveIsotopicPatternGenerator;



fn elements() -> ChemicalComposition<'static> {
    "(H12O6C6)12".parse().unwrap()
}


fn baffling_gen(composition: ChemicalComposition) {
    let mut gen = BafflingRecursiveIsotopicPatternGenerator::new();
    gen.isotopic_variants(composition, 0, 0, PROTON);
}


fn composition_scaling(c: &mut Criterion) {
    c.bench_function("BafflingRecursiveIsotopicPatternGenerator", |b| {
        b.iter(|| baffling_gen(black_box(elements())))
    });

    let mut gen = BafflingRecursiveIsotopicPatternGenerator::new();
    c.bench_function("Re-using Generator", |b| {
        b.iter(|| gen.isotopic_variants(black_box(elements()), 0, 0, PROTON))
    });
}

criterion_group!(benches, composition_scaling);
criterion_main!(benches);
