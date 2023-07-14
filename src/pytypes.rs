use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;


#[pyclass]
#[derive(Clone,Debug)]
pub struct WrappedDatum(marlowe_lang::types::marlowe::MarloweDatum);

#[pymethods]
impl WrappedDatum {
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_string(&self) -> String {
        format!("{:?}",self.0)
    }
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
            Ok(c) => Ok(WrappedDatum(c)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }
    }
}

#[pyclass]
#[derive(Clone,Debug)]
pub struct WrappedCase(marlowe_lang::types::marlowe::Case);

#[pymethods]
impl WrappedCase {
    #[pyo3(text_signature = "($self, f)")]
    #[staticmethod]
    fn notify_on_true() -> Self {
        WrappedCase(marlowe_lang::types::marlowe::Case { 
            case: Some(marlowe_lang::types::marlowe::Action::Notify { 
                notify_if: Some(marlowe_lang::types::marlowe::Observation::True) 
            }), 
            then: Some(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(
                Box::new(marlowe_lang::types::marlowe::Contract::Close)
            ))
        })
    }
}

#[pyclass]
#[derive(Clone,Debug)]
pub struct WrappedContract(marlowe_lang::types::marlowe::Contract);

#[pymethods]
impl WrappedContract {

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
    fn close() -> Self {
       WrappedContract(marlowe_lang::types::marlowe::Contract::Close)
    }

    #[staticmethod]
    fn from_marlowe_dsl(dsl:&str,variables:Vec<(String, i64)>) -> PyResult<Self> {
       match marlowe_lang::types::marlowe::Contract::from_dsl(dsl,variables) {
        Ok(c) => Ok(WrappedContract(c)),
        Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }       
    }

    #[staticmethod]
    fn from_json(json:&str) -> PyResult<Self> {
       match marlowe_lang::types::marlowe::Contract::from_json(json) {
        Ok(c) => Ok(WrappedContract(c)),
        Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }       
    }

    #[staticmethod]
    fn from_cbor_hex(cbor_hex:&str) -> PyResult<Self> {
       match marlowe_lang::extras::utils::try_decode_cborhex_marlowe_plutus_contract(cbor_hex) {
            Ok(c) => Ok(WrappedContract(c)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }
    }

    #[staticmethod]
    fn when(case: Vec<WrappedCase>, contract: WrappedContract, timeout: i64) -> PyResult<Self> {
        let mut cc = vec![];
        for x in case {
            cc.push(Some(marlowe_lang::types::marlowe::Case { 
                case: x.0.case, 
                then: x.0.then
            }))
        }
        Ok(WrappedContract(marlowe_lang::types::marlowe::Contract::When { 
            when: cc, 
            timeout: Some(marlowe_lang::types::marlowe::Timeout::TimeConstant(timeout)), 
            timeout_continuation: Some(Box::new(contract.0))
        }))
    }  

}


