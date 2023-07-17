use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

pub(crate) mod token;
pub(crate) mod contract;
pub(crate) mod datum;
pub(crate) mod case;
pub(crate) mod value;
pub(crate) mod party;
pub(crate) mod bound;
pub(crate) mod observation;
pub(crate) mod timeout;
pub(crate) mod payee;

pub use token::Token;
pub use payee::Payee;
pub use party::Party;

pub use datum::Datum;
pub use value::Value;
pub use bound::Bound;
pub use case::Case;
pub use contract::Contract;
pub use contract::PossiblyMerkleizedContract;

pub use observation::Observation;
pub use timeout::Timeout;

pub(crate)fn to_py_err(s:&str) -> PyErr {
    PyValueError::new_err(String::from(s))
}