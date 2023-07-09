use std::collections::HashMap;
use pyo3::{prelude::*, types::{PyDict, PyType}};
use marlowe_lang::{*, types::marlowe::Contract};

#[pyfunction]
fn marlowe_to_json(marlowe_dsl:&str,variables:HashMap<String, i64>) -> PyResult<String> {
    match marlowe_lang::extras::utils::try_marlowe_to_json(marlowe_dsl,&variables) {
        Ok(s) => PyResult::Ok(s),
        Err(e) => Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(e))
    }
}


struct WrappedContract(Contract);


impl<'a> FromPyObject<'a> for WrappedContract {
    fn extract(ob: &'a PyAny) -> PyResult<Self> {
        let dict: &PyDict = ob.extract()?;
        let field1: i32 = dict.get_item("field1").unwrap().extract()?;
        let field2: String = dict.get_item("field2").unwrap().extract()?;
        Ok(WrappedContract(Contract::Close))
    }
}

impl IntoPy<PyObject> for WrappedContract {
    fn into_py(self, py: Python) -> PyObject {
        let dict = PyDict::new(py);
        dict.set_item("field1", "hello").unwrap();
        dict.to_object(py)
    }
}

#[pyfunction]
fn example() -> PyResult<WrappedContract> {
    Ok(WrappedContract(Contract::When { when: vec![], timeout: None, timeout_continuation: None }))
}



// EXPORT ALL THE THINGS
#[pymodule]
fn marlowepy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(marlowe_to_json, m)?)?;
    m.add_function(wrap_pyfunction!(example, m)?)?;
    Ok(())
}