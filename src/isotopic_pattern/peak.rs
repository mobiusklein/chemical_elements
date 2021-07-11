
#[derive(Debug)]
pub struct Peak {
    pub mz: f64,
    pub intensity: f64,
    pub charge: i32,
}


pub type PeakList = Vec<Peak>;
