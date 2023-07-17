use marlowe_lang::types::marlowe::*;
    

fn opt_py<T, F>(option: &Option<T>, f: F) -> String
where
    T: Sized,
    F: FnOnce(&T) -> String,
{
    match option {
        Some(x) => f(x),
        None => "null".into(),
    }
}

pub fn datum_as_python(x:&MarloweDatum) -> String {
    format!("Datum.from_contract(contract={},role_payout_validator_hash=\"{}\")",
        contract_as_python(&x.contract),
        x.marlowe_params.0
    )
}

pub fn payee_as_python(x:&Payee) -> String {
    match &x {
        Payee::Account(p) => {
            match p {
                Some(inner_party) => format!("Payee.Account({})",party_as_python(inner_party)),
                None => "Payee.Account(null)".to_string(),
            }
        },
        Payee::Party(p) => {
            match p {
                Some(inner_party) => format!("Payee.Party({})",party_as_python(inner_party)),
                None => "Payee.Party(null)".to_string(),
            }
        }
    }
}

pub fn observation_as_python(x:&Observation) -> String {
    match &x {
        Observation::AndObs { both: _, and: _ } => 
            todo!(),
        Observation::OrObs { either: _, or: _ } => 
            todo!(),
        Observation::NotObs { not: _ } => 
            todo!(),
        Observation::ChoseSomething(_) => 
            todo!(),
        Observation::ValueGE { value: _, ge_than: _ } => 
            todo!(),
        Observation::ValueGT { value: _, gt_than: _ } => 
            todo!(),
        Observation::ValueLT { value: _, lt_than: _ } => 
            todo!(),
        Observation::ValueLE { value: _, le_than: _ } => 
            todo!(),
        Observation::ValueEQ { value: _, equal_to: _ } => 
            todo!(),
        Observation::True => 
            "Observation.FalseObs()".into(),
        Observation::False => 
            "Observation.TrueObs()".into(),
    }
}

pub fn value_box_as_python(x:&Box<Value>) -> String {value_as_python(x)}
pub fn value_as_python(x:&Value) -> String {
    match &x {
        Value::AvailableMoney(_a, _b) => {
            todo!()
        },
        Value::ConstantValue(_x) => {
            todo!()
        },
        Value::NegValue(_x) => {
            todo!()
        },
        Value::AddValue(_a, _b) => {
            todo!()
        },
        Value::SubValue(_a, _b) => {
            todo!()
        },
        Value::MulValue(_a, _b) => {
            todo!()
        },
        Value::DivValue(_a, _b) => {
            todo!()
        },
        Value::ChoiceValue(_x) => todo!(),
        Value::TimeIntervalStart => "Value.TimeIntervalStart()".into(),
        Value::TimeIntervalEnd => "Value.TimeIntervalEnd()".into(),
        Value::UseValue(_x) => {
            todo!()
        },
        Value::Cond(_a,_b,_c) => {
            todo!()
        },
        Value::ConstantParam(x) => {
            format!("Value.ConstantParam(\"{x}\")")
        },
    }
}

pub fn token_as_python(x:&Token) -> String {
    if x.token_name.is_empty() && x.currency_symbol.is_empty() {
        "Token.ADA()".to_string()
    } else {
        format!("Token.Custom(token_name=\"{}\",currency_symbol=\"{}\")",x.token_name,x.currency_symbol)
    }
}

pub fn bound_as_python(x:&Bound) -> String {
    format!("Bound({},{})",x.0,x.1)
}
pub fn bounds_as_python(x:&Vec<Bound>) -> String {
    let bounds = x.iter().map(|x|format!("Bound({},{})",x.0,x.1)).collect::<Vec<String>>();
    let comma_separated_bounds = bounds.join(",");
    format!("[{comma_separated_bounds}]")
}
pub fn party_as_python(x:&Party) -> String {
    match &x {
        marlowe_lang::types::marlowe::Party::Address(addr) => 
            format!("Party.Address(\"{}\")",addr.as_bech32().unwrap_or("invalid_address!".into())),
        marlowe_lang::types::marlowe::Party::Role { role_token } => 
            format!("Party.Role(\"{role_token}\")"),
    }
}

pub fn case_as_python(x:&Case) -> String {
    
    let continue_with_py = match &x.then {
        Some(c) => {
            match c {
                PossiblyMerkleizedContract::Raw(contract_continuation) => {
                    format!("PossiblyMerkleizedContract.raw(contract={})",contract_box_as_python(contract_continuation))
                },
                PossiblyMerkleizedContract::Merkleized(m) => {
                    format!("PossiblyMerkleizedContract.merkleized(hash=\"{m}\")")
                },
            }
        },
        None => "null".into(),
    };

    match x.case.as_ref() {
        Some(marlowe_lang::types::marlowe::Action::Notify { notify_if }) => {
            // observation:Observation,then_continue_with:Contract
            match notify_if {
                Some(_obs) => {
                    let obs_py = "null"; //Observation(obs.clone()).as_python();
                    let con_py = "null";
                    format!("Case.Notify(observation={obs_py},then_continue_with={con_py})")
                },
                None => "Case.Notify(observation=null)".to_string(),
            }
        },
        Some(marlowe_lang::types::marlowe::Action::Deposit { into_account, party, of_token, deposits }) => {
            let into_account_py = opt_py(into_account,party_as_python);
            let party_py = opt_py(party,party_as_python);
            let of_token_py = opt_py(of_token,token_as_python);
            let deposits_py = opt_py(deposits,value_as_python);
            // fn deposit(into_account:Party,by:Party,of_token:Token,value:Value,then_continue_with:Contract) -> Case {
            format!("Case.Deposit(into_account={into_account_py},by={party_py},of_token={of_token_py},value={deposits_py},then_continue_with={continue_with_py})")
        },
        Some(marlowe_lang::types::marlowe::Action::Choice { for_choice, choose_between }) => {
            
            let (choice_name_py,choice_owner_py) : (String,String) = match for_choice {
                Some(ChoiceId { choice_name, choice_owner }) => {
                    (
                        format!("\"{}\"",choice_name),
                        match choice_owner {
                            Some(o) => party_as_python(o),
                            _ => "null".into()
                        }
                    )
                },
                None => ("null".into(),"null".into()),
            };

            let bounds = choose_between.iter().flatten().map(|x|x.to_owned()).collect::<Vec<Bound>>();
            let bounds_py = opt_py(&Some(bounds),bounds_as_python);

            // fn choice(choice_name:&str,choice_owner:Party,bounds:Vec<Bound>,then_continue_with:PossiblyMerkleizedContract) -> Case {
            format!("Case.Choice(choice_name={choice_name_py}, choice_owner={choice_owner_py},bounds={bounds_py},then_continue_with={continue_with_py})")
        },
        None => "null".into()
    }        
}

pub fn timeout_as_python(x:&Timeout) -> String {
    match &x {
        Timeout::TimeConstant(n) => format!("Timeout.TimeConstant({n})"),
        Timeout::TimeParam(p) => format!("Timeout.TimeParam(\"{p}\")"),
    }
}

pub fn contract_box_as_python(x:&Box<Contract>) -> String { contract_as_python(x) }
pub fn contract_as_python(x:&Contract) -> String {
    
    match x{
        marlowe_lang::types::marlowe::Contract::Close => String::from("Contract.Close()"),
        marlowe_lang::types::marlowe::Contract::Pay { from_account, to, token, pay, then } => {
            
            let token_py = opt_py(token,token_as_python);
            let from_account_py = opt_py(from_account,party_as_python);
            let to_py = opt_py(to,payee_as_python);
            let pay_py = opt_py(pay,value_as_python);
            let then_py = opt_py(then,contract_box_as_python);
            
            format!("Contract.Pay(from_account_of={from_account_py},to={to_py},token={token_py},pay={pay_py},then={then_py})")
        },
        marlowe_lang::types::marlowe::Contract::If { x_if, then, x_else } => {
            let if_py = opt_py(x_if,observation_as_python);
            let then_py = opt_py(then,contract_box_as_python);
            let else_py = opt_py(x_else,contract_box_as_python);
            format!("Contract.If(observation={if_py},then={then_py},else={else_py})")
        },
        marlowe_lang::types::marlowe::Contract::When { when, timeout, timeout_continuation } => {
            
            let timeout_py = opt_py(timeout,timeout_as_python);
            let timeout_continuation_py = opt_py(timeout_continuation,contract_box_as_python);
            let cases_py = when.iter().map(|x|opt_py(x,case_as_python)).collect::<Vec<String>>().join(",");
            format!("Contract.When(case=[{cases_py}],timeout={timeout_py},timeout_continuation={timeout_continuation_py})")
        },
        marlowe_lang::types::marlowe::Contract::Let { x_let, be, then } => {
            let ValueId::Name(var_name) = x_let;
            let be_py = opt_py(be,value_box_as_python);
            let then_py = opt_py(then,contract_box_as_python);
            format!("Contract.Let(variable_name=\"{var_name}\"),be={be_py},then={then_py})")
        },
        marlowe_lang::types::marlowe::Contract::Assert { assert, then } => {
            let assert_py = opt_py(assert,observation_as_python);
            let then_py = opt_py(then,contract_box_as_python);
            format!("Contract.Assert(observation={assert_py},then={then_py})")
        },
    }
}
