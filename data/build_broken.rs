/// A script to dynamically generate the periodic table constant in crate::table.
/// Reads the isotopic distribution data from data/nist_mass.json.
use serde_json::{Map, Value};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path;
use std::process;

fn load_from_file(path: &path::Path) -> Map<String, Value> {
    let mut reader = fs::File::open(path).unwrap();
    let mut buf: String = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let val: Value = serde_json::from_str(&buf).unwrap();
    let m: Map<String, Value> = val.as_object().unwrap().clone();
    m
}

//' A Dummy copy of the implementation for convenience.
#[derive(Default, Clone, Copy)]
struct Isotope {
    pub mass: f64,
    pub abundance: f64,
    pub neutrons: u16,
    pub neutron_shift: i8,
}

fn write_prelude(buffer: &mut Cursor<Vec<u8>>) {
    write!(
        buffer,
        r#"
use lazy_static::lazy_static;
use crate::element::{{Element, Isotope, PeriodicTable}};


pub fn populate_periodic_table(table: &mut PeriodicTable) {{
"#
    )
    .unwrap();
}

fn prepare_element(buffer: &mut Cursor<Vec<u8>>, symbol: &String, isotopes: &Value) {
    let mut isos: Vec<Isotope> = Vec::new();
    let iso = isotopes.as_object().unwrap();

    let mut reference_entry = Isotope {
        ..Default::default()
    };

    let mut mono_neutrons = None;

    // TODO: Account for the elements whose monoisotopic variant is given by reference like `Ac`
    for (_isotope_i, (isonum, mass_abundance)) in iso.iter().enumerate() {
        let mass_abundance = mass_abundance.as_array().unwrap();
        let isonum: u16 = isonum.parse().unwrap();
        let mass = &mass_abundance[0];
        let abundance = mass_abundance[1].as_f64().unwrap_or_else(|| {
            panic!(
                "Failed to convert the isotopic abundance of {} ({})",
                isonum, &mass_abundance[1]
            )
        });

        let mut iso = if isonum == 0 {
            if mass.is_i64() {
                mono_neutrons = mass.as_i64().and_then(|val| {
                    if val < 0 {
                        panic!("The monoisotopic neutrons for {} is {}", symbol, val)
                    } else {
                        Some(val as u16)
                    }
                });
            }
            continue
        } else {
            Isotope {
                mass: mass.as_f64().unwrap_or_else(|| {
                    panic!(
                        "Failed to convert the isotopic mass of to float {} ({})",
                        isonum, mass
                    )
                }),
                abundance: abundance,
                neutrons: isonum,
                ..Default::default()
            }
        };
        if iso.abundance == 0.0 {
            if mono_neutrons.is_some() && mono_neutrons.unwrap() == iso.neutrons {
                iso.abundance = 1.0;
                reference_entry = iso;
            } else {
                continue;
            }
        }
        if isonum == 0 {
            reference_entry = iso;
        } else {
            isos.push(iso);
        }
    }
    isos.sort_by_key(|i| (i.abundance * 100.0).round() as i32);
    let n = isos.len();
    if n > 0 {
        let most_abundant_neutron_count = isos[n - 1].neutrons;
        let most_abundant_mass = isos[n - 1].mass;
        let mut element_number = 0;
        for y in &mut isos {
            y.neutron_shift = ((y.neutrons as i32) - (most_abundant_neutron_count as i32)) as i8;
            if y.neutron_shift == 0 {
                element_number = y.neutrons / 2;
            }
        }
        writeln!(buffer, "\n\tlet mut elt = Element {{ symbol: String::from(\"{}\"), most_abundant_isotope: {}, most_abundant_mass: {:.6}, element_number: {},..Default::default() }};",
                 symbol, most_abundant_neutron_count, most_abundant_mass, element_number).unwrap();
        for y in &isos {
            writeln!(buffer, "\telt.isotopes.insert({}, Isotope {{ mass: {:.6}, abundance: {:.6}, neutrons: {}, neutron_shift: {} }});",
                        y.neutrons, y.mass, y.abundance, y.neutrons, y.neutron_shift).unwrap();
        }
    } else {
        writeln!(buffer, "\tlet mut elt = Element {{ symbol: String::from(\"{}\"), most_abundant_isotope: {}, most_abundant_mass: {:.6}, ..Default::default() }};",
                 symbol, 0, reference_entry.mass).unwrap();
        writeln!(buffer, "\telt.isotopes.insert({}, Isotope {{ mass: {:.6}, abundance: {:.6}, neutrons: {}, neutron_shift: {} }});",
                    reference_entry.neutrons, reference_entry.mass, reference_entry.abundance, reference_entry.neutrons,
                    reference_entry.neutron_shift).unwrap();
    }
    writeln!(buffer, "\telt.index_isotopes();\n\ttable.add(elt);\n").unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src/table.rs");
    let elements = load_from_file(path::Path::new("data/nist_mass.json"));
    let mut buffer = Cursor::new(Vec::new());
    write_prelude(&mut buffer);
    for (key, val) in elements.iter() {
        prepare_element(&mut buffer, key, val);
    }
    write!(
        &mut buffer,
        r#"}}

lazy_static! {{
    pub static ref PERIODIC_TABLE: PeriodicTable = {{
        let mut t = PeriodicTable::new();
        populate_periodic_table(&mut t);
        t
    }};
}}"#
    )
    .unwrap();
    buffer.set_position(0);
    let mut out = String::new();
    buffer.read_to_string(&mut out).unwrap();
    let mut destination = fs::File::create("src/table.rs").unwrap();
    destination.write_all(out.as_bytes()).unwrap();
    process::Command::new("rustfmt").arg("src/table.rs").status().unwrap();
}
