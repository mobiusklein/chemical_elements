
pub const PROTON: f64 = 1.007276;


pub fn mass_charge_ratio(neutral_mass: f64, z: i32, charge_carrier: f64) -> f64 {
    let zf: f64 = z as f64;
    return (neutral_mass + (zf * charge_carrier)) / zf.abs();
}


pub fn neutral_mass(mz: f64, z: i32, charge_carrier: f64) -> f64 {
    let zf: f64 = z as f64;
    return (mz * zf.abs()) - (zf * charge_carrier);
}
