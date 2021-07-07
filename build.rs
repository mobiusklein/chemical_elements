/// A script to dynamically generate the periodic table constant in crate::table.
/// Reads the isotopic distribution data from data/nist_mass.json.
use serde_json;
use serde_json::{Map, Value};
use std::fs;
use std::io::{Cursor, Read, Write};
use std::path;

fn load_from_file(path: &path::Path) -> Map<String, Value> {
    let mut reader = fs::File::open(path).unwrap();
    let mut buf: String = String::new();
    reader.read_to_string(&mut buf).unwrap();
    let val: Value = serde_json::from_str(&buf).unwrap();
    let m: Map<String, Value> = val.as_object().unwrap().clone();
    return m;
}

//' A Dummy copy of the implementation for convenience.
#[derive(Default)]
struct Isotope {
    pub mass: f64,
    pub abundance: f64,
    pub neutrons: u16,
    pub neutron_shift: i8,
}


fn write_prelude(buffer: &mut Cursor<Vec<u8>>) {
    write!(buffer, r#"
use lazy_static::lazy_static;
use crate::element::{{Element, Isotope, PeriodicTable}};


pub fn populate_periodic_table(table: &mut PeriodicTable) {{
"#).unwrap();
}


fn prepare_element(buffer: &mut Cursor<Vec<u8>>, symbol: &String, isotopes: &Value) {
    let mut isos: Vec<Isotope> = Vec::new();
    let iso = isotopes.as_object().unwrap();
    let mut reference_entry = Isotope {
        ..Default::default()
    };
    for (isonum, mass_abundance) in iso.iter() {
        let mass_abundance = mass_abundance.as_array().unwrap();
        let isonum: u16 = isonum.parse().unwrap();
        let x = Isotope {
            mass: mass_abundance[0].as_f64().unwrap(),
            abundance: mass_abundance[1].as_f64().unwrap(),
            neutrons: isonum,
            ..Default::default()
        };
        if isonum == 0 {
            reference_entry = x;
        } else {
            isos.push(x);
        }
    }
    isos.sort_by_key(|i| (i.abundance * 100.0).round() as i32);
    let n = isos.len();
    if n > 0 {
        let most_abundant_neutron_count = isos[n - 1].neutrons;
        let most_abundant_mass = isos[n - 1].mass;
        for y in &mut isos {
            y.neutron_shift = ((y.neutrons as i32) - (most_abundant_neutron_count as i32)) as i8;
        }
        writeln!(buffer, "\n\tlet mut elt = Element {{ symbol: String::from(\"{}\"), most_abundant_isotope: {}, most_abundant_mass: {:.6}, ..Default::default() }};",
                 symbol, most_abundant_neutron_count, most_abundant_mass).unwrap();
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
    writeln!(buffer, "\ttable.add(elt);\n").unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=src/table.rs");
    let elements = load_from_file(path::Path::new("data/nist_mass.json"));
    let mut buffer = Cursor::new(Vec::new());
    write_prelude(&mut buffer);
    for (key, val) in elements.iter() {
        prepare_element(&mut buffer, key, val);
    }
    write!(&mut buffer, r#"}}

lazy_static! {{
    pub static ref PERIODIC_TABLE: PeriodicTable = {{
        let mut t = PeriodicTable::new();
        populate_periodic_table(&mut t);
        t
    }};
}}"#).unwrap();
    buffer.set_position(0);
    let mut out = String::new();
    buffer.read_to_string(&mut out).unwrap();
    let mut destination = fs::File::create("src/table.rs").unwrap();
    destination.write(out.as_bytes()).unwrap();
}
