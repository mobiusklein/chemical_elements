use std::ops::Index;

use pyo3::types::{PyMapping, PyString, PyTuple};
use pyo3::PyTypeCheck;
use pyo3::{
    exceptions::{PyTypeError, PyValueError},
    prelude::*,
};

use chemical_elements::isotopic_pattern::baffling::IsotopicDistribution;
use chemical_elements::{
    isotopic_pattern::Peak, parse_formula, ChemicalComposition, ElementSpecification,
    PERIODIC_TABLE, PROTON,
};

#[pyclass(module = "pychemical_elements", name = "ChemicalComposition", mapping)]
#[derive(Default, Clone)]
pub struct PyChemicalComposition {
    inner: ChemicalComposition<'static>,
}

impl<'py> TryFrom<FormulaOrMapping<'py>> for PyChemicalComposition {
    fn try_from(value: FormulaOrMapping<'py>) -> PyResult<Self> {
        value.convert()
    }

    type Error = PyErr;
}

pub enum FormulaOrMapping<'py> {
    Formula(String),
    Mapping(pyo3::Bound<'py, PyMapping, >),
    Composition(PyRef<'py, PyChemicalComposition>),
}

impl<'source> FromPyObject<'source> for FormulaOrMapping<'source> {
    fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Self> {
        if ob.is_instance_of::<PyString>() {
            Ok(FormulaOrMapping::Formula(ob.extract::<String>()?))
        } else if ob.is_instance_of::<PyChemicalComposition>() {
            let cob = ob.extract()?;
            Ok(FormulaOrMapping::Composition(cob))
        } else if PyMapping::type_check(ob) && ob.hasattr("items")? {
            let cob = ob.clone().downcast_into::<PyMapping>()?;
            Ok(FormulaOrMapping::Mapping(cob))
        } else {
            Err(PyTypeError::new_err(
                "Failed to coerce object to formula or mapping",
            ))
        }
    }
}

impl FormulaOrMapping<'_> {
    pub fn convert(&self) -> PyResult<PyChemicalComposition> {
        match self {
            FormulaOrMapping::Mapping(value) => {
                let mut this = ChemicalComposition::default();
                let items = value.items()?;
                for kve in items {
                    let kv: &Bound<'_, PyTuple> = kve.downcast()?;
                    let elem_str_py = kv.get_borrowed_item(0)?;
                    let elem_str = elem_str_py.extract()?;
                    let count = kv.get_item(1)?.extract()?;
                    this.inc_str(elem_str, count);
                }
                Ok(PyChemicalComposition { inner: this })
            }
            FormulaOrMapping::Formula(value) => {
                if let Ok(inner) = parse_formula(value) {
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
    #[pyo3(signature = (formula=None))]
    pub fn new(formula: Option<FormulaOrMapping>) -> PyResult<Self> {
        if let Some(formula) = formula {
            formula.try_into()
        } else {
            Ok(PyChemicalComposition::default())
        }
    }

    fn __repr__(&self) -> String {
        format!("PyChemicalComposition(\"{}\")", self.inner)
    }

    pub fn __str__(&self) -> String {
        self.inner.to_string()
    }

    pub fn __getitem__(&self, key: &str) -> i32 {
        *self.inner.index(key)
    }

    pub fn __setitem__(&mut self, key: &str, val: i32) {
        self.inner[key] = val;
    }

    pub fn fmass(&mut self) -> f64 {
        self.inner.fmass()
    }

    pub fn copy(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    #[getter]
    pub fn mass(&self) -> f64 {
        self.inner.mass()
    }

    pub fn __iadd__(&mut self, other: &Self) {
        self.inner += &other.inner
    }

    fn __add__(&self, other: &Self) -> Self {
        Self {
            inner: &self.inner + &other.inner,
        }
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
        Self {
            inner: &self.inner * i,
        }
    }

    fn __imul__(&mut self, i: i32) {
        self.inner *= i;
    }

    fn __contains__(&self, elt: &str) -> bool {
        self.inner.get_str(elt) != 0
    }

    fn keys(&self) -> Vec<String> {
        self.inner
            .iter()
            .map(|(k, _)| k.element.symbol.to_string())
            .collect()
    }

    fn values(&self) -> Vec<i32> {
        self.inner.iter().map(|(_, v)| *v).collect()
    }

    fn items(&self) -> Vec<(String, i32)> {
        self.inner
            .iter()
            .map(|(k, v)| (k.element.symbol.to_string(), *v))
            .collect()
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyChemIter {
        PyChemIter {
            inner: slf.keys().into_iter(),
        }
    }
}

#[pyclass(module = "pychemical_elements")]
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

#[pyclass(module = "pychemical_elements", name = "Peak")]
pub struct PyPeak(Peak);

#[pymethods]
impl PyPeak {
    #[new]
    fn new(mz: f64, intensity: f64) -> Self {
        Self(Peak {
            mz,
            intensity,
        })
    }

    #[getter]
    fn get_mz(&self) -> f64 {
        self.0.mz
    }

    #[setter]
    fn set_mz(&mut self, value: f64) {
        self.0.mz = value
    }

    #[getter]
    fn get_intensity(&self) -> f64 {
        self.0.intensity
    }

    #[setter]
    fn set_intensity(&mut self, value: f64) {
        self.0.intensity = value
    }

    fn __repr__(&self) -> String {
        format!("{}", self.0)
    }
}

impl From<Peak> for PyPeak {
    fn from(value: Peak) -> Self {
        Self(value)
    }
}

#[pyfunction]
fn isotopic_variants(
    mut composition: PyChemicalComposition,
    npeaks: i32,
    charge: i32,
) -> PyResult<Vec<PyPeak>> {
    let inner = composition.inner;
    let dist = IsotopicDistribution::from_composition(inner, npeaks - 1);
    let isotopic_peaks = dist.isotopic_variants(charge, PROTON);
    composition.inner = dist.composition;
    Ok(isotopic_peaks.into_iter().map(|p| p.into()).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pychemical_elements(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(isotopic_variants, m)?)?;
    m.add_class::<PyChemicalComposition>()?;
    m.add_class::<PyPeak>()?;
    Ok(())
}
