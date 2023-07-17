use super::*;
use pyo3::{exceptions::PyValueError, pyclass, pymethods, PyResult};

#[pyclass]
#[derive(Clone, Debug)]
pub struct Observation(pub(crate) marlowe_lang::types::marlowe::Observation);

#[pymethods]
impl Observation {

    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::observation_as_python(&self.0)
    }


    #[pyo3(text_signature = "($self, f)")]
    pub fn as_string(&self) -> String {
        format!("{:?}", self.0)
    }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}", v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}", e))),
        }
    }

    #[staticmethod]
    #[pyo3(name = "AndObs")]
    pub fn and_observation(both: Self, and: Self) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::AndObs {
            both: Some(Box::new(both.0)),
            and: Some(Box::new(and.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "ChoseSomething")]
    pub fn chose_something(choice_name: &str, party: Party) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ChoseSomething(
            Some(marlowe_lang::types::marlowe::ChoiceId {
                choice_name: choice_name.into(),
                choice_owner: Some(party.0),
            }),
        ))
    }

    #[staticmethod]
    #[pyo3(name = "FalseObs")]
    pub fn false_observation() -> Self {
        Self(marlowe_lang::types::marlowe::Observation::False)
    }

    #[staticmethod]
    #[pyo3(name = "NotObs")]
    pub fn not_observation(not: Self) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::NotObs {
            not: Some(Box::new(not.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "OrObs")]
    pub fn or_observation(either: Observation, or: Observation) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::OrObs {
            either: Some(Box::new(either.0)),
            or: Some(Box::new(or.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "TrueObs")]
    pub fn true_observation() -> Self {
        Self(marlowe_lang::types::marlowe::Observation::True)
    }

    #[staticmethod]
    #[pyo3(name = "ValueEQ")]
    pub fn value_equals(value: Value, equal_to: Value) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ValueEQ {
            value: Some(Box::new(value.0)),
            equal_to: Some(Box::new(equal_to.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "ValueGE")]
    pub fn value_greater_than_or_equal_to(value: Value, greater_or_equal_to: Value) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ValueGE {
            value: Some(Box::new(value.0)),
            ge_than: Some(Box::new(greater_or_equal_to.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "ValueGT")]
    pub fn value_greater_than(value: Value, greater_than: Value) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ValueGT {
            value: Some(Box::new(value.0)),
            gt_than: Some(Box::new(greater_than.0)),
        })
    }

    #[staticmethod]
    #[pyo3(name = "ValueLE")]
    pub fn value_less_or_equal_to(value: Value, less_than: Value) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ValueLE {
            value: Some(Box::new(value.0)),
            le_than: Some(Box::new(less_than.0)),
        })
    }
    #[staticmethod]
    #[pyo3(name = "ValueLT")]
    pub fn value_less_than(value: Value, lt: Value) -> Self {
        Self(marlowe_lang::types::marlowe::Observation::ValueLT {
            value: Some(Box::new(value.0)),
            lt_than: Some(Box::new(lt.0)),
        })
    }
}
