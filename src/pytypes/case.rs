use pyo3::{PyResult, pyclass, pymethods, exceptions::PyValueError};
use super::*;

#[pyclass]
#[derive(Clone,Debug)]
pub struct Case(pub(crate)marlowe_lang::types::marlowe::Case);

#[pymethods]
impl Case {

    #[pyo3(text_signature = "($self, f)")] pub fn as_string(&self) -> String { format!("{:?}",self.0) }
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_json(&self) -> PyResult<String> {
        match marlowe_lang::serialization::json::serialize(&self.0) {
            Ok(v) => Ok(format!("{}",v)),
            Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
        }        
    }

    #[staticmethod]
    #[pyo3(name="Notify")]
    fn notify(observation:Observation,then_continue_with:Contract) -> Self {
        Case(marlowe_lang::types::marlowe::Case { 
            case: Some(marlowe_lang::types::marlowe::Action::Notify { 
                notify_if: Some(observation.0) 
            }), 
            then: Some(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(
                Box::new(then_continue_with.0)
            ))
        })
    }

    #[staticmethod]
    #[pyo3(name="Choice")]
    fn choice(choice_name:&str,choice_owner:Party,bounds:Vec<Bound>,then_continue_with:Contract) -> Case {
        Case(
            marlowe_lang::types::marlowe::Case { 
                case: Some(marlowe_lang::types::marlowe::Action::Choice { 
                    for_choice: Some(marlowe_lang::types::marlowe::ChoiceId {
                        choice_name: choice_name.into(),
                        choice_owner: Some(choice_owner.0)
                    }), 
                    choose_between: bounds.iter().map(|x| 
                        marlowe_lang::types::marlowe::Bound(x.0.0,x.0.1)).map(Some).collect()
                }),
                then: Some(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(Box::new(then_continue_with.0)))
        })
    }

    #[staticmethod]
    #[pyo3(name="Deposit")]
    fn deposit(into_account:Party,by:Party,of_token:Token,value:Value,then_continue_with:Contract) -> Case {
        Case(
            marlowe_lang::types::marlowe::Case { 
                case: Some(marlowe_lang::types::marlowe::Action::Deposit { 
                    into_account: Some(into_account.0), 
                    party: Some(by.0), 
                    of_token: Some(of_token.0), 
                    deposits: Some(value.0) 
                }     
            ),
                then: Some(marlowe_lang::types::marlowe::PossiblyMerkleizedContract::Raw(Box::new(then_continue_with.0)))
        })
    }
}
