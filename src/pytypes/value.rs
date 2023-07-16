use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;

#[pyclass]
#[derive(Clone,Debug)]
pub struct Value(pub(crate)marlowe_lang::types::marlowe::Value);

#[pymethods]
impl Value {
    #[pyo3(text_signature = "($self, f)")] pub fn as_string(&self) -> String { format!("{:?}",self.0) }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }
    #[staticmethod]
    #[pyo3(name="Add")]
    fn add(value_one:Value,value_two:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::AddValue(
                Some(Box::new(value_one.0)),
                Some(Box::new(value_two.0))
            )
        )
    }
    #[staticmethod]
    #[pyo3(name="Divide")]
    fn divide(value:Value,by:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::DivValue(
                Some(Box::new(value.0)),
                Some(Box::new(by.0))
            )
        )
    }
    #[staticmethod]
    #[pyo3(name="Constant")]
    fn constant(value:i64) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::ConstantValue(value)
        )
    }
    #[staticmethod]
    #[pyo3(name="ConstantParam")]
    fn parameter(value:&str) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::ConstantParam(value.into())
        )
    }
}
