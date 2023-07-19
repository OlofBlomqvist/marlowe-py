use pyo3::{pyclass, pymethods, PyResult, exceptions::PyValueError};

#[pyclass]
#[derive(Clone,Debug)]
pub struct Token(pub(crate)marlowe_lang::types::marlowe::Token);

#[pymethods]
impl Token {

    
    #[pyo3(text_signature = "($self, f)")]
    pub fn as_python(&self) -> String {
        crate::code_gen::token_as_python(&self.0)
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

    #[staticmethod]
    fn from_json(json:&str) -> PyResult<Self> {
       match marlowe_lang::deserialization::json::deserialize::<marlowe_lang::types::marlowe::Token>(json.into()) {
        Ok(c) => Ok(Self(c)),
        Err(e) => Err(PyValueError::new_err(format!("did not work! {:?}",e)))
       }       
    }

    #[new]
    fn new(token_name: &str, currency_symbol: &str) -> Self {
        Self(marlowe_lang::types::marlowe::Token {
            currency_symbol: currency_symbol.into(),
            token_name: token_name.into()
        })
    }

    #[staticmethod]
    #[pyo3(name="ADA")]
    fn ada() -> Self {
        Self(marlowe_lang::types::marlowe::Token::ada())
    }

    #[staticmethod]
    #[pyo3(name="Custom")]
    fn custom(token_name: &str, currency_symbol: &str) -> Self {
        Self::new(token_name,currency_symbol)
    }

}