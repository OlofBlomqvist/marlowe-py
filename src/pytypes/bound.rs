use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};

#[pyclass]
#[derive(Clone,Debug)]
pub struct Bound(pub(crate)marlowe_lang::types::marlowe::Bound);

#[pymethods]
impl Bound {
    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::bound_as_python(&self.0)
    }

    #[pyo3(text_signature = "($self, f)")] pub fn as_string(&self) -> String { format!("{:?}",self.0) }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }
    #[new]
    fn new(from: i128, to: i128) -> Self {
        Bound(marlowe_lang::types::marlowe::Bound(from,to))
    }

}
