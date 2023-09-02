use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;

#[pyclass]
#[derive(Clone,Debug)]
pub struct Value(pub(crate)marlowe_lang::types::marlowe::Value);

#[pymethods]
impl Value {
    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::value_as_python(&self.0)
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
    #[pyo3(name="AddValue")]
    fn add(value_one:Value,value_two:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::AddValue(
                Some(Box::new(value_one.0)),
                Some(Box::new(value_two.0))
            )
        )
    }

    #[staticmethod]
    #[pyo3(name="DivValue")]
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
    fn constant(value:i128) -> Self {
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

    #[staticmethod]
    #[pyo3(name="AvailableMoney")]
    fn available_money(account_of:Party,token:Token) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::AvailableMoney(
                Some(account_of.0),
                Some(token.0)
            )
        )
    }

    #[staticmethod]
    #[pyo3(name="NegValue")]
    fn neg_value(negate:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::NegValue(Some(Box::new(negate.0)))
        )
    }

    #[staticmethod]
    #[pyo3(name="SubValue")]
    fn sub_value(subtract:Value,from_val:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::SubValue(
                Some(Box::new(subtract.0)), 
                Some(Box::new(from_val.0)
            ))
        )
    }

    #[staticmethod]
    #[pyo3(name="MulValue")]
    fn mul_value(multiply:Value,by:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::MulValue(
                Some(Box::new(multiply.0)), 
                Some(Box::new(by.0)
            ))
        )
    }

    #[staticmethod]
    #[pyo3(name="ChoiceValue")]
    fn choice_value(choice_name:&str,choice_owner:Party) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::ChoiceValue(
                Some(marlowe_lang::types::marlowe::ChoiceId { 
                    choice_name: choice_name.into(), 
                    choice_owner: Some(choice_owner.0) 
                })
            )
        )
    }

    #[staticmethod]
    #[pyo3(name="TimeIntervalStart")]
    fn time_interval_start() -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::TimeIntervalStart
        )
    }

    #[staticmethod]
    #[pyo3(name="TimeIntervalEnd")]
    fn time_interval_end() -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::TimeIntervalEnd
        )
    }

    #[staticmethod]
    #[pyo3(name="UseValue")]
    fn use_value(value_name:&str) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::UseValue(
                marlowe_lang::types::marlowe::ValueId::Name(value_name.into())
            )
        )
    }

    #[staticmethod]
    #[pyo3(name="Cond")]
    fn condition(if_obs:Observation,then_val:Value,else_val:Value) -> Self {
        Self(
            marlowe_lang::types::marlowe::Value::Cond(
                Some(if_obs.0),
                Some(Box::new(then_val.0)),
                Some(Box::new(else_val.0))
            )
        )
    }

}
