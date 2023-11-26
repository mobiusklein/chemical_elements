use criterion::{black_box, criterion_group, criterion_main, Criterion};

use chemical_elements::{ChemicalCompositionMap, ChemicalCompositionVec, ElementSpecification, ChemicalComposition};


fn elements() -> Vec<(ElementSpecification<'static>, i32)> {
    let hydrogen = "H".parse().unwrap();
    let oxygen = "O".parse().unwrap();
    let carbon = "C".parse().unwrap();
    return vec![
        (hydrogen, 12),
        (oxygen, 6),
        (carbon, 6)
    ]
}


fn hashmap(elements: Vec<(ElementSpecification, i32)>) {
    let mut comp = ChemicalCompositionMap::new();
    for (k, v) in elements {
        comp.set(k, v);
    }
    comp *= 2;
    comp.mass();
}

fn vec(elements: Vec<(ElementSpecification, i32)>) {
    let mut comp = ChemicalCompositionVec::new();
    for (k, v) in elements {
        comp.set(k, v);
    }
    comp *= 2;
    comp.mass();
}


fn r#abstract(elements: Vec<(ElementSpecification, i32)>) {
    let mut comp = ChemicalComposition::new();
    for (k, v) in elements {
        comp.set(k, v);
    }
    comp *= 2;
    comp.mass();
}


fn composition_scaling(c: &mut Criterion) {
    c.bench_function("hashmap", |b| {
        b.iter(|| hashmap(black_box(elements())))
    });
    c.bench_function("vec", |b| {
        b.iter(|| vec(black_box(elements())))
    });
    c.bench_function("abstract", |b| {
        b.iter(|| r#abstract(black_box(elements())))
    });
}

criterion_group!(benches, composition_scaling);
criterion_main!(benches);
