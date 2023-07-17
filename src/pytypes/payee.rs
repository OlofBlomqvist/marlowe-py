use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;


#[pyclass]
#[derive(Clone,Debug)]
pub struct Payee(pub(crate)marlowe_lang::types::marlowe::Payee);

#[pymethods]
impl Payee {

    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::payee_as_python(&self.0)
    }


    #[pyo3(text_signature = "($self, f)")] pub fn as_string(&self) -> String { format!("{:?}",self.0) }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }

    #[staticmethod]
    #[pyo3(name="Party")]
    pub fn party(party:Party) -> Payee {
        Payee(marlowe_lang::types::marlowe::Payee::Party(Some(party.0)))
    }
    
    #[staticmethod]
    #[pyo3(name="Account")]
    pub fn role(party:Party) -> Payee {
        Payee(marlowe_lang::types::marlowe::Payee::Account(Some(party.0)))
    }
}