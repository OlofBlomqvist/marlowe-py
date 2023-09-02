
use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;

#[pyclass]
#[derive(Clone,Debug)]
pub struct PossiblyMerkleizedContract(pub(crate)marlowe_lang::types::marlowe::PossiblyMerkleizedContract);

#[pymethods]
impl PossiblyMerkleizedContract {
    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        match &self.0 {
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(c) => {
                format!("PossiblyMerkleizedContract.Raw({})",crate::code_gen::contract_box_as_python(c))
            },
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Merkleized(r) => {
                format!("PossiblyMerkleizedContract.Merkleized({r})")
            },
        }
    }

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_string(&self) -> String {
        format!("{:?}",self.0)
    }

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_marlowe_dsl(&self) -> String  {
        match &self.0 {
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(c) => {
                crate::code_gen::contract_box_as_python(c)
            },
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Merkleized(r) => {
                format!("\"{r}\"")
            },
        }
    }

    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match serde_json::to_string_pretty(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(_e) => Err(PyValueError::new_err("e:?".to_string()))
        }
    }   


    #[pyo3(text_signature = "($self, f)")]
    pub fn show_status(&self) -> PyResult<String> {
        match &self.0 {
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(c) => Contract(*c.clone()).show_status(),
            marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Merkleized(m) => Ok(format!("Merkleized contract: '{m}'")),
        }
    }

    
    #[staticmethod]
    pub fn merkleized(hash:&str) -> Self {
       Self(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Merkleized(hash.into()))
    }
    #[staticmethod]
    pub fn raw(contract:Contract) -> Self {
       Self(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(Box::new(contract.0)))
    }
}

#[pyclass]
#[derive(Clone,Debug)]
pub struct Contract(pub(crate)marlowe_lang::types::marlowe::Contract);

#[pymethods]
impl Contract {
    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::contract_as_python(&self.0)
    }

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
    fn from_marlowe_dsl(dsl:&str,variables:Vec<(String, i128)>) -> PyResult<Self> {
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
    fn when(case: Vec<Case>, timeout: Timeout, timeout_continuation: Contract) -> PyResult<Self> {
        
        let mut cases = vec![];
        for x in case {
            cases.push(Some(x.0))
        }

        Ok(Contract(marlowe_lang::types::marlowe::Contract::When { 
            when: cases, 
            timeout: Some(timeout.0), 
            timeout_continuation: Some(Box::new(timeout_continuation.0))
        }))
    }  
            
    #[staticmethod]
    #[pyo3(name="Close")]
    fn close() -> Self {
       Contract(marlowe_lang::types::marlowe::Contract::Close)
    }

    
    #[staticmethod]
    #[pyo3(name="Assert")]
    fn assert(obs:Observation,then:Contract) -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Assert { 
            assert: Some(obs.0), 
            then: Some(Box::new(then.0)) 
        })
    }

    
    #[staticmethod]
    #[pyo3(name="If")]
    fn r#if(obs:Observation,then:Contract,else_contract:Contract) -> Self {
        Self(marlowe_lang::types::marlowe::Contract::If { 
            x_if: Some(obs.0), 
            then: Some(Box::new(then.0)), 
            x_else: Some(Box::new(else_contract.0)) 
        })
    }

    #[staticmethod]
    #[pyo3(name="Let")]
    fn r#let(variable_name:&str, be: Value, then: Contract) -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Let { 
            x_let: marlowe_lang::types::marlowe::ValueId::Name(variable_name.into()), 
            be: Some(Box::new(be.0)), 
            then: Some(Box::new(then.0)) 
        })
    }

    #[staticmethod]
    #[pyo3(name="Pay")]
    fn pay(from_account_of:Party,to:Payee,token:Token,pay:Value,then:Contract) -> Self {
        Self(marlowe_lang::types::marlowe::Contract::Pay { 
            from_account: Some(from_account_of.0), 
            to: Some(to.0), 
            token: Some(token.0), 
            pay: Some(pay.0), 
            then: Some(Box::new(then.0)) 
        })
    }



}


