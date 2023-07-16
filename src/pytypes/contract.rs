use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;


#[pyclass]
#[derive(Clone,Debug)]
pub struct Contract(pub(crate)marlowe_lang::types::marlowe::Contract);

#[pymethods]
impl Contract {

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_string(&self) -> String {
        format!("{:?}",self.0)
    }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_marlowe_dsl(&self) -> String {
        format!("{}",self.0.to_dsl())
    }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match self.0.to_json() {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(e))
        }
    }   


    #[staticmethod]
    fn from_marlowe_dsl(dsl:&str,variables:Vec<(String, i64)>) -> PyResult<Self> {
       match marlowe_lang::types::marlowe::Contract::from_dsl(dsl,variables) {
        Ok(c) => Ok(Contract(c)),
        Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }       
    }

    #[staticmethod]
    fn from_json(json:&str) -> PyResult<Self> {
       match marlowe_lang::types::marlowe::Contract::from_json(json) {
        Ok(c) => Ok(Contract(c)),
        Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }       
    }

    #[staticmethod]
    fn from_cbor_hex(cbor_hex:&str) -> PyResult<Self> {
       match marlowe_lang::extras::utils::try_decode_cborhex_marlowe_plutus_contract(cbor_hex) {
            Ok(c) => Ok(Contract(c)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }
    }


    #[pyo3(text_signature = "($self, f)")]
    pub fn show_status(&self) -> PyResult<String> {
        let instance = marlowe_lang::semantics::ContractInstance::new(&self.0.clone(), None);
        match marlowe_lang::semantics::ContractSemantics::process(&instance) {
            Ok((_,state)) => match state {
                marlowe_lang::semantics::MachineState::Closed => Ok("Closed".into()),
                marlowe_lang::semantics::MachineState::Faulted(e) => Err(to_py_err(&e)),
                marlowe_lang::semantics::MachineState::ContractHasTimedOut => Ok("timed out".into()),
                marlowe_lang::semantics::MachineState::WaitingForInput { expected, timeout } => {
                    Ok(format!("waiting for input until {timeout}: \n {:?}", expected))
                },
                marlowe_lang::semantics::MachineState::ReadyForNextStep => todo!(),
            }
            Err(e) => Err(to_py_err(&format!("{:?}",e)))
        }   
    }


    
    #[staticmethod]
    #[pyo3(name="When")]
    fn when(case: Vec<Case>, contract: Contract, timeout: Timeout) -> PyResult<Self> {
        let mut cc = vec![];
        for x in case {
            cc.push(Some(marlowe_lang::types::marlowe::Case { 
                case: x.0.case, 
                then: x.0.then
            }))
        }
        Ok(Contract(marlowe_lang::types::marlowe::Contract::When { 
            when: cc, 
            timeout: Some(timeout.0), 
            timeout_continuation: Some(Box::new(contract.0))
        }))
    }  
        
    #[staticmethod]
    #[pyo3(name="Close")]
    fn close() -> Self {
       Contract(marlowe_lang::types::marlowe::Contract::Close)
    }

    #[staticmethod]
    #[pyo3(name="Assert")]
    fn assert() -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Assert { 
            assert: None, 
            then: None 
        })
    }

    #[staticmethod]
    #[pyo3(name="If")]
    fn r#if() -> Self {
        Self(marlowe_lang::types::marlowe::Contract::If { 
            x_if: None, 
            then: None, 
            x_else: None 
        })
    }

    #[staticmethod]
    #[pyo3(name="Let")]
    fn r#let() -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Let { 
            x_let: marlowe_lang::types::marlowe::ValueId::Name("hello".into()), 
            be:None, 
            then: None 
        })
    }

    #[staticmethod]
    #[pyo3(name="Pay")]
    fn pay() -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Pay { 
            from_account: None, 
            to: None, 
            token: None, 
            pay: None, 
            then: None 
        })
    }



}


