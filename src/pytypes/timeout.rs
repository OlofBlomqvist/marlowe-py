use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;


#[pyclass]
#[derive(Clone,Debug)]
pub struct Timeout(pub(crate)marlowe_lang::types::marlowe::Timeout);

#[pymethods]
impl Timeout {

    #[pyo3(text_signature = "($self, f)")] pub fn as_string(&self) -> String { format!("{:?}",self.0) }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }

    
    #[staticmethod]
    #[pyo3(name="TimeConstant")]
    pub fn posix(time:i64) -> Self {
        Self(marlowe_lang::types::marlowe::Timeout::TimeConstant(time))
    }

    #[staticmethod]
    #[pyo3(name="TimeParam")]
    pub fn time_param(time:&str) -> Self {
        Self(marlowe_lang::types::marlowe::Timeout::TimeParam(time.into()))
    }
}