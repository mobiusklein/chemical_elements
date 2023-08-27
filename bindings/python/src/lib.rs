use pyo3::{prelude::*, exceptions::{PyValueError, PyTypeError}};
use pyo3::types::{PyMapping, PyUnicode};
use pyo3::ffi::PyMapping_Check;

use chemical_elements::{ChemicalComposition, PERIODIC_TABLE, parse_formula, ElementSpecification, PROTON};
use chemical_elements::isotopic_pattern::baffling::IsotopicDistribution;


#[pyclass(module="pychemical_elements")]
#[derive(Default, Clone)]
pub struct PyChemicalComposition {
    inner: ChemicalComposition<'static>
}

impl<'py> TryFrom<FormulaOrMapping<'py>> for PyChemicalComposition {
    fn try_from(value: FormulaOrMapping<'py>) -> PyResult<Self> {
        value.convert()
    }

    type Error = PyErr;
}

pub enum FormulaOrMapping<'py> {
    Formula(String),
    Mapping(&'py PyMapping),
    Composition(PyRef<'py, PyChemicalComposition>)
}


impl<'source> FromPyObject<'source> for FormulaOrMapping<'source> {
    fn extract(ob: &'source PyAny) -> PyResult<Self> {
        if ob.is_instance_of::<PyUnicode>()? {
            Ok(FormulaOrMapping::Formula(ob.extract::<String>()?))
        }
        else if ob.is_instance_of::<PyChemicalComposition>()? {
            let cob = ob.extract()?;
            Ok(FormulaOrMapping::Composition(cob))
        }
        else if unsafe {PyMapping_Check(ob.into_ptr()) == 1} && ob.hasattr("items")? {
            let cob = ob.cast_as::<PyMapping>()?;
            Ok(FormulaOrMapping::Mapping(cob))
        }
        else {
            Err(PyTypeError::new_err("Failed to coerce object to formula or mapping"))
        }
    }
}

impl<'py> FormulaOrMapping<'py> {
    pub fn convert(&self) -> PyResult<PyChemicalComposition> {
        match self {
            FormulaOrMapping::Mapping(value) => {
                let mut this = ChemicalComposition::default();
                let items = value.items()?.iter()?;
                for kve in items {
                    let kv = kve?;
                    let (elem_str, count): (&str, i32) = kv.extract()?;
                    this.inc_str(elem_str, count);
                }
                Ok(PyChemicalComposition { inner: this })
            },
            FormulaOrMapping::Formula(value) => {
                if let Ok(inner) = parse_formula(&value) {
                    let mut this = ChemicalComposition::default();
                    for (k, v) in inner.iter() {
                        let elt = PERIODIC_TABLE.get(&k.element.symbol).unwrap();
                        let my_key = ElementSpecification::new(elt, k.isotope);
                        this.inc(my_key, *v);
                    }
                    Ok(PyChemicalComposition { inner: this })
                } else {
                    Err(PyValueError::new_err(format!("Invalid formula {}", value)))
                }
            }
            FormulaOrMapping::Composition(value) => {
                let this = value.copy();
                Ok(this)
            }
        }
    }
}


#[pymethods]
impl PyChemicalComposition {
    #[new]
    pub fn new(formula: Option<FormulaOrMapping>) -> PyResult<Self> {
        if let Some(formula) = formula {
            formula.try_into()
        } else {
            Ok(PyChemicalComposition::default())
        }
    }

    fn __repr__(&self) -> String {
        format!("PyChemicalComposition(\"{}\")", self.inner.to_string())
    }

    pub fn __str__(&self) -> String {
        self.inner.to_string()
    }

    pub fn __getitem__(&self, key: &str) -> i32 {
        self.inner.get_str(key)
    }

    pub fn __setitem__(&mut self, key: &str, val: i32) {
        self.inner[key] = val;
    }

    pub fn fmass(&mut self) -> f64 {
        self.inner.fmass()
    }

    pub fn copy(&self) -> Self {
        Self { inner: self.inner.clone() }
    }

    #[getter]
    pub fn mass(&self) -> f64 {
        self.inner.mass()
    }

    pub fn __iadd__(&mut self, other: &Self) {
        self.inner += &other.inner
    }

    fn __add__(&self, other: &Self) -> Self {
        Self { inner: &self.inner + &other.inner }
    }

    fn __sub__(&self, other: &Self) -> Self {
        let mut tmp = self.inner.clone();
        tmp -= &other.inner;
        Self { inner: tmp }
    }

    fn __isub__(&mut self, other: &Self) {
        self.inner -= &other.inner;
    }

    fn __mul__(&self, i: i32) -> Self {
        Self { inner: &self.inner * i }
    }

    fn __imul__(&mut self, i: i32) {
        self.inner *= i;
    }

    fn __contains__(&self, elt: &str) -> bool {
        self.inner.get_str(elt) != 0
    }

    fn keys(&self) -> Vec<String> {
        self.inner.iter().map(|(k, _)| { k.element.symbol.to_string() }).collect()
    }

    fn values(&self) -> Vec<i32> {
        self.inner.iter().map(|(_, v)| { *v }).collect()
    }

    fn items(&self) -> Vec<(String, i32)> {
        self.inner.iter().map(|(k, v)| { (k.element.symbol.to_string(), *v) }).collect()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyChemIter {
        PyChemIter { inner: slf.keys().into_iter() }
    }
}


#[pyclass(module="pychemical_elements")]
struct PyChemIter {
    inner: std::vec::IntoIter<String>,
}


#[pymethods]
impl PyChemIter {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<String> {
        slf.inner.next()
    }
}



#[pyfunction(module="pychemical_elements")]
fn pyisotopic_variants<'a>(mut composition: PyChemicalComposition, npeaks: i32, charge: i32) -> PyResult<Vec<f64>> {
    let inner = composition.inner;
    let dist = IsotopicDistribution::from_composition(inner, npeaks - 1);
    let isotopic_peaks = dist.isotopic_variants(charge, PROTON);
    composition.inner = dist.composition;
    Ok(isotopic_peaks.iter().map(|p| p.intensity).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pychemical_elements(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyisotopic_variants, m)?)?;
    m.add_class::<PyChemicalComposition>()?;
    Ok(())
}