use pyo3::prelude::*;

pub mod pytypes;

#[pyfunction]
pub fn test() -> String {
    String::from("HELLO FROM RUST")
}

#[pymodule]
#[pyo3(name = "marlowe")]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test, m)?)?;
    m.add_class::<pytypes::WrappedCase>()?;
    m.add_class::<pytypes::WrappedContract>()?;
    m.add_class::<pytypes::WrappedDatum>()?;
    Ok(())
}

