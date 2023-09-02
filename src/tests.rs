#[cfg(test)]
use std::process::Command;

#[cfg(test)]
use std::io::Write;


#[cfg(test)]
fn call_python_script(file_name:&str) -> Result<String, String> {
    let output = Command::new("python")
        .arg(file_name)
        .output()
        .expect("Failed to execute command");
    if !output.stderr.is_empty() {
        Err(String::from_utf8(output.stderr).map_err(|e|format!("{e:?}"))?)

    } else {
        String::from_utf8(output.stdout).map_err(|e|format!("{e:?}"))
    }
}


// this test is to make sure that generated code produces 
// the same dsl back as the original input we genrated it from
#[test]
fn as_python() {

    let temp_file_name = "temp_test_code_gen.py";
    let mut temp_file = std::fs::File::create(temp_file_name).unwrap();

    let dsl = r#"When [
        (Case
           (Deposit (Role "Lender") (Role "Lender")
              (Token "" "")
              (ConstantParam "Amount"))
           (Pay (Role "Lender")
              (Party (Role "Borrower"))
              (Token "" "")
              (ConstantParam "Amount")
              (When [
                 (Case
                    (Deposit (Role "Borrower") (Role "Borrower")
                       (Token "" "")
                       (AddValue
                          (ConstantParam "Interest")
                          (ConstantParam "Amount")))
                    (Pay (Role "Borrower")
                       (Party (Role "Lender"))
                       (Token "" "")
                       (AddValue
                          (ConstantParam "Interest")
                          (ConstantParam "Amount")) Close))] (TimeParam "Payback deadline") Close)))] (TimeParam "Loan deadline") (When [
        (Case
           (Deposit (Role "Party") (Role "Party")
              (Token "" "")
              (ConstantParam "Amount paid by party"))
           (When [
              (Case
                 (Deposit (Role "Counterparty") (Role "Counterparty")
                    (Token "" "")
                    (ConstantParam "Amount paid by counterparty"))
                 (When [] (TimeParam "First window beginning")
                    (When [
                       (Case
                          (Choice
                             (ChoiceId "Price in first window" (Role "Oracle")) [
                             (Bound 0 1000000000)])
                          (When [] (TimeParam "Second window beginning")
                             (When [
                                (Case
                                   (Choice
                                      (ChoiceId "Price in second window" (Role "Oracle")) [
                                      (Bound 0 1000000000)])
                                   (If
                                      (ValueGT
                                         (ChoiceValue
                                            (ChoiceId "Price in first window" (Role "Oracle")))
                                         (ChoiceValue
                                            (ChoiceId "Price in second window" (Role "Oracle"))))
                                      (Let "Decrease in price"
                                         (SubValue
                                            (ChoiceValue
                                               (ChoiceId "Price in first window" (Role "Oracle")))
                                            (ChoiceValue
                                               (ChoiceId "Price in second window" (Role "Oracle"))))
                                         (Pay (Role "Counterparty")
                                            (Account (Role "Party"))
                                            (Token "" "")
                                            (Cond
                                               (ValueLT
                                                  (UseValue "Decrease in price")
                                                  (ConstantParam "Amount paid by counterparty"))
                                               (UseValue "Decrease in price")
                                               (ConstantParam "Amount paid by counterparty")) Close))
                                      (If
                                         (ValueLT
                                            (ChoiceValue
                                               (ChoiceId "Price in first window" (Role "Oracle")))
                                            (ChoiceValue
                                               (ChoiceId "Price in second window" (Role "Oracle"))))
                                         (Let "Increase in price"
                                            (SubValue
                                               (ChoiceValue
                                                  (ChoiceId "Price in second window" (Role "Oracle")))
                                               (ChoiceValue
                                                  (ChoiceId "Price in first window" (Role "Oracle"))))
                                            (Pay (Role "Party")
                                               (Account (Role "Counterparty"))
                                               (Token "" "")
                                               (Cond
                                                  (ValueLT
                                                     (UseValue "Increase in price")
                                                     (ConstantParam "Amount paid by party"))
                                                  (UseValue "Increase in price")
                                                  (ConstantParam "Amount paid by party")) Close)) Close)))] (TimeParam "Second window deadline") Close)))] (TimeParam "First window deadline") Close)))] (TimeParam "Counterparty deposit deadline") Close))] (TimeParam "Party deposit deadline") Close)"#.trim();

    let parse_result = marlowe_lang::deserialization::marlowe::deserialize(dsl)
        .expect("should be possible to deserialize basic contract..");

    let pythonized = crate::pytypes::Contract(parse_result.contract).as_python();
    
    // run the generated code and convert the result to marlowe dsl before returning it to us, so that we can compare with the original contract.
    let py_script = format!("from marlowe import *\nx={pythonized}\nprint(x.as_marlowe_dsl())");

    println!("generated python code: {}",&py_script);
    
    writeln!(temp_file, "{}",py_script).expect("should be able to save to temp file");
    let result = call_python_script(temp_file_name);
    std::fs::remove_file(temp_file_name).unwrap_or_default();
    match result {
        Ok(result) => {
            let mut inputs = std::collections::HashMap::<String,i128>::new();
            for x in &parse_result.uninitialized_const_params {
                inputs.insert(x.clone(),42);
            }
            for x in &parse_result.uninitialized_time_params {
                inputs.insert(x.clone(),88888888);
            }
            let python_result_dsl_converted_to_json = 
                marlowe_lang::deserialization::marlowe::deserialize_with_input(&result,inputs.clone())
                    .expect("should be able to parse the python result.")
                    .contract.to_json().unwrap();
            let original_dsl_converted_to_json = 
                marlowe_lang::deserialization::marlowe::deserialize_with_input(dsl,inputs.clone())
                    .expect("should be able to parse the python result.")
                    .contract.to_json().unwrap();
            assert!(
                python_result_dsl_converted_to_json==original_dsl_converted_to_json
            )
        },
        Err(err) => panic!("code gen is not working as expected! {err}")
    }
    
}
