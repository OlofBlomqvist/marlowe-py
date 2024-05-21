use pyo3::prelude::*;
use pytypes::PossiblyMerkleizedContract;
pub mod pytypes;
mod tests;
mod code_gen;

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

#[pyfunction]
pub fn raw(contract:crate::pytypes::Contract) -> PossiblyMerkleizedContract {
    PossiblyMerkleizedContract::raw(contract)
}

#[pyfunction]
pub fn merkleized(hash:&str) -> PossiblyMerkleizedContract {
    PossiblyMerkleizedContract::merkleized(hash)
}

#[pyfunction]
pub fn version() -> String {
    "marlowe-py: 0.1.6, marlowe-rs: 0.3.2".into()
}




#[pyfunction]
pub fn inputs_json_to_cbor(json:&str) -> PyResult<String> {

    let deserialized : Vec<marlowe_lang::types::marlowe::InputAction> = 
        marlowe_lang::deserialization::json::deserialize(json.into()).map_err(|e|pytypes::to_py_err(&e))?;

    let action = deserialized.into_iter()
        .map(marlowe_lang::types::marlowe::PossiblyMerkleizedInput::Action)
        .collect::<Vec<marlowe_lang::types::marlowe::PossiblyMerkleizedInput>>();

    marlowe_lang::serialization::cborhex::serialize(action).map_err(|e|pytypes::to_py_err(&e))

}


#[pymodule]
#[pyo3(name = "marlowe")]
pub fn rust(_py: Python, m: &PyModule) -> PyResult<()> {
    
    m.add_function(wrap_pyfunction!(inputs_json_to_cbor,m)?)?;

    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(merkleized, m)?)?;
    m.add_function(wrap_pyfunction!(raw, m)?)?;
    m.add_function(wrap_pyfunction!(decode_redeemer_from_cbor_hex, m)?)?;
    m.add_function(wrap_pyfunction!(try_decode_any_cbor_hex, m)?)?;
    m.add_class::<pytypes::Case>()?;
    m.add_class::<pytypes::Contract>()?;
    m.add_class::<pytypes::PossiblyMerkleizedContract>()?;
    m.add_class::<pytypes::Datum>()?;
    m.add_class::<pytypes::Token>()?;
    m.add_class::<pytypes::Party>()?;
    m.add_class::<pytypes::Payee>()?;
    m.add_class::<pytypes::Value>()?;
    m.add_class::<pytypes::Bound>()?;
    m.add_class::<pytypes::Observation>()?;
    m.add_class::<pytypes::Timeout>()?;
    
    Ok(())
}
