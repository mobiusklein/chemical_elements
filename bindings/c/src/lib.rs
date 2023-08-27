use std::os::raw::c_char;
use std::ptr;
use std::ffi::CStr;
use chemical_elements::{ChemicalComposition, ElementSpecification};


#[derive(Default)]
pub struct CChemicalComposition(ChemicalComposition<'static>);


#[no_mangle]
pub extern "C" fn parse_formula(formula: *mut c_char, out: *mut *mut CChemicalComposition) -> u32 {
    unsafe {
        *out = ptr::null_mut();
    }
    unsafe {
        let formula_view = CStr::from_ptr(formula);
        let encoded_view = formula_view.to_string_lossy();

        match ChemicalComposition::parse(&encoded_view) {
            Ok(composition) => {
                *out = Box::into_raw(Box::new(CChemicalComposition(composition)));
                0
            },
            Err(parse_err) => {
                (parse_err as u32) + 1
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn free_chemical_composition(slf: *mut CChemicalComposition) -> u32 {
    unsafe { drop(Box::from_raw(slf)) };
    0
}


impl CChemicalComposition {
    #[no_mangle]
    pub extern "C" fn new(out: *mut *mut CChemicalComposition) -> u32 {
        unsafe {
            *out = ptr::null_mut();
            *out = Box::into_raw(Box::new(CChemicalComposition::default()))
        }
        0
    }

    #[no_mangle]
    pub extern "C" fn mass(&self) -> f64 {
        self.0.mass()
    }

    #[no_mangle]
    pub extern "C" fn copy(&self, out: *mut *mut CChemicalComposition) -> u32 {
        unsafe { *out = ptr::null_mut(); }
        unsafe { *out = Box::into_raw(Box::new(CChemicalComposition(self.0.clone()))); }
        0
    }

    #[no_mangle]
    pub extern "C" fn get(&self, element_spec: *mut c_char) -> i32 {
        unsafe {
            let spec_view = CStr::from_ptr(element_spec);
            let encoded_view = spec_view.to_string_lossy();
            self.0.get_str(&encoded_view)
        }
    }

    #[no_mangle]
    pub extern "C" fn set(&mut self, element_spec: *mut c_char, count: i32) -> u32 {
        unsafe {
            let spec_view = CStr::from_ptr(element_spec);
            let encoded_view = spec_view.to_string_lossy();
            match encoded_view.parse::<ElementSpecification>() {
                Ok(spec) => {
                    self.0.set(spec, count);
                    0
                },
                Err(e) => {
                    e as u32 + 1
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn increment(&mut self, element_spec: *mut c_char, count: i32) -> u32 {
        unsafe {
            let spec_view = CStr::from_ptr(element_spec);
            let encoded_view = spec_view.to_string_lossy();
            match encoded_view.parse::<ElementSpecification>() {
                Ok(spec) => {
                    self.0.inc(spec, count);
                    0
                },
                Err(e) => {
                    e as u32 + 1
                }
            }
        }
    }

    #[no_mangle]
    pub extern "C" fn add(&mut self, other: &CChemicalComposition) -> u32 {
        self.0 += &other.0;
        0
    }

    #[no_mangle]
    pub extern "C" fn subtract(&mut self, other: &CChemicalComposition) -> u32 {
        self.0 -= &other.0;
        0
    }

    #[no_mangle]
    pub extern "C" fn scale(&mut self, other: i32) -> u32 {
        self.0 *= other;
        0
    }
}
