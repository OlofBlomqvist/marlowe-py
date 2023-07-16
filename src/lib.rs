use pyo3::prelude::*;
pub mod pytypes;

#[pyfunction]
pub fn decode_redeemer_from_cbor_hex(cbor_hex:&str) -> PyResult<String> {
    match marlowe_lang::extras::utils::try_decode_redeemer_input_cbor_hex(cbor_hex) {
        Ok(v) => Ok(format!("{:?}",v)),
        Err(e) => Err(pytypes::to_py_err(&e)),
    }
}

#[pyfunction]
pub fn try_decode_any_cbor_hex(cbor_hex:&str) -> PyResult<String> {
    match marlowe_lang::extras::utils::try_decode_any_marlowe_data(cbor_hex) {
        Ok(v) => Ok(v),
        Err(e) => Err(pytypes::to_py_err(&e)),
    }
}

#[pymodule]
#[pyo3(name = "marlowe")]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(decode_redeemer_from_cbor_hex, m)?)?;
    m.add_function(wrap_pyfunction!(try_decode_any_cbor_hex, m)?)?;
    m.add_class::<pytypes::Case>()?;
    m.add_class::<pytypes::Contract>()?;
    m.add_class::<pytypes::Datum>()?;
    m.add_class::<pytypes::Token>()?;
    m.add_class::<pytypes::Party>()?;
    m.add_class::<pytypes::Value>()?;
    m.add_class::<pytypes::Bound>()?;
    m.add_class::<pytypes::Observation>()?;
    m.add_class::<pytypes::Timeout>()?;
    
    Ok(())
}
