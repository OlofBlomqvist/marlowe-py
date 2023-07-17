use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;


#[pyclass]
#[derive(Clone,Debug)]
pub struct Datum(pub(crate)marlowe_lang::types::marlowe::MarloweDatum);

#[pymethods]
impl Datum {

    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::datum_as_python(&self.0)
    }


    #[pyo3(text_signature = "($self, f)")] 
    pub fn as_string(&self) -> String { format!("{:?}",self.0) }

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_cbor_hex(&self) -> PyResult<String> {
        match marlowe_lang::serialization::cborhex::serialize(self.0.clone()) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }

    #[staticmethod]
    fn from_cbor_hex(cbor_hex:&str) -> PyResult<Self> {
       match marlowe_lang::extras::utils::try_decode_cborhex_marlowe_plutus_datum(cbor_hex) {
            Ok(c) => Ok(Datum(c)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }
    }
    
    #[pyo3(text_signature = "($self, f)")]
    pub fn show_status(&self) -> PyResult<String> {
        let instance = marlowe_lang::semantics::ContractInstance::from_datum(&self.0.clone());
        match marlowe_lang::semantics::ContractSemantics::process(&instance) {
            Ok((_,state)) => match state {
                marlowe_lang::semantics::MachineState::Closed => Ok("Closed".into()),
                marlowe_lang::semantics::MachineState::Faulted(e) => Err(to_py_err(&e)),
                marlowe_lang::semantics::MachineState::ContractHasTimedOut => Ok("timed out".into()),
                marlowe_lang::semantics::MachineState::WaitingForInput { expected, timeout } => {
                    Ok(format!("waiting for input until {timeout}: \n {:?}", expected))
                },
                marlowe_lang::semantics::MachineState::ReadyForNextStep => Err(to_py_err("invalid - probably a bug in marlowe-rs")),
            }
            Err(e) => Err(to_py_err(&format!("{:?}",e)))
        }   
    }
   
    #[staticmethod]
    pub fn from_contract(contract:Contract,role_payout_validator_hash:&str) -> Self {
        let instance = marlowe_lang::semantics::ContractInstance::new(&contract.0, Some(role_payout_validator_hash.into()));
        Self(instance.as_datum())
    }
}