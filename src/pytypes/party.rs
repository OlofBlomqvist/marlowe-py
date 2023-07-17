use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};


#[pyclass]
#[derive(Clone,Debug)]
pub struct Party(pub(crate)marlowe_lang::types::marlowe::Party);

#[pymethods]
impl Party {

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
      crate::code_gen::party_as_python(&self.0)
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
    #[pyo3(name="Address")]
    pub fn address(addr:&str) -> Party {
        Party(marlowe_lang::types::marlowe::Party::Address(marlowe_lang::types::marlowe::Address::from_bech32(addr).unwrap()))
    }
    
    #[staticmethod]
    #[pyo3(name="Role")]
    pub fn role(token_name:&str) -> Party {
        Party(marlowe_lang::types::marlowe::Party::role(token_name))
    }
}