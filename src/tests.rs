use std::convert::Infallible;
use std::process::{Command, Output};
use std::io::Write;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{SystemTime, Duration};


fn call_python_script(file_name:&str) -> Result<String, String> {
    let output = Command::new("python")
        .arg(file_name)
        .output()
        .expect("Failed to execute command");
    if output.stderr.len() > 0 {
        Err(String::from_utf8(output.stderr).map_err(|e|format!("{e:?}"))?)

    } else {
        String::from_utf8(output.stdout).map_err(|e|format!("{e:?}"))
    }
}


// todo: include one of each construct!
// this test is too make sure that generated code produces 
// the same dsl back as the original input we genrated it from
#[test]
fn as_python() {

    let temp_file_name = "temp_test_code_gen.py";
    let mut temp_file = std::fs::File::create(&temp_file_name.clone()).unwrap();

    let dsl = r#"
    When [
  (Case
     (Deposit (Role "Seller") (Role "Buyer")
        (Token "" "")
        (ConstantParam "Price"))
     (When [
           (Case
              (Choice
                 (ChoiceId "Everything is alright" (Role "Buyer")) [
                 (Bound 0 0)]) Close)
           ,
           (Case
              (Choice
                 (ChoiceId "Report problem" (Role "Buyer")) [
                 (Bound 1 1)])
              (Pay (Role "Seller")
                 (Account (Role "Buyer"))
                 (Token "" "")
                 (ConstantParam "Price")
                 (When [
                       (Case
                          (Choice
                             (ChoiceId "Confirm problem" (Role "Seller")) [
                             (Bound 1 1)]) Close)
                       ,
                       (Case
                          (Choice
                             (ChoiceId "Dispute problem" (Role "Seller")) [
                             (Bound 0 0)])
                          (When [
                                (Case
                                   (Choice
                                      (ChoiceId "Dismiss claim" (Role "Mediator")) [
                                      (Bound 0 0)])
                                   (Pay (Role "Buyer")
                                      (Party (Role "Seller"))
                                      (Token "" "")
                                      (ConstantParam "Price") Close))
                                ,
                                (Case
                                   (Choice
                                      (ChoiceId "Confirm problem" (Role "Mediator")) [
                                      (Bound 1 1)]) Close)] (TimeParam "Mediation deadline") Close))] (TimeParam "Complaint response deadline") Close)))] (TimeParam "Complaint deadline") Close))] (TimeParam "Payment deadline") Close
    "#.trim();

    let parse_result = marlowe_lang::deserialization::marlowe::deserialize(dsl)
        .expect("should be possible to deserialize basic contract..");

    let pythonized = crate::pytypes::Contract(parse_result.contract).as_python();
    
    // run the generated code and convert the result to marlowe dsl before returning it to us, so that we can compare with the original contract.
    let py_script = format!("from marlowe import *\nx={pythonized}\nprint(x.as_marlowe_dsl())");

    println!("generated python code: {}",&py_script);
    
    writeln!(temp_file, "{}",py_script).expect("should be able to save to temp file");
    let result = call_python_script(&temp_file_name);
    _ = std::fs::remove_file(temp_file_name).unwrap_or_default();
    match result {
        Ok(result) => {
            let mut inputs = std::collections::HashMap::<String,i64>::new();
            inputs.insert("Price".into(), 42);
            inputs.insert("Mediation deadline".into(),999);
            inputs.insert("Complaint response deadline".into(),11);
            inputs.insert("Complaint deadline".into(),999);
            inputs.insert("Payment deadline".into(),11111111);
            let python_result_dsl_converted_to_json = 
                marlowe_lang::deserialization::marlowe::deserialize_with_input(&result,inputs.clone())
                    .expect("should be able to parse the python result.")
                    .contract.to_json().unwrap();
            let original_dsl_converted_to_json = 
                marlowe_lang::deserialization::marlowe::deserialize_with_input(&dsl,inputs.clone())
                    .expect("should be able to parse the python result.")
                    .contract.to_json().unwrap();
            assert!(
                python_result_dsl_converted_to_json==original_dsl_converted_to_json
            )
        },
        Err(err) => panic!("code gen is not working as expected! {err}")
    }
    
}
