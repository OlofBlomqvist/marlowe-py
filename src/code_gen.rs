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

pub fn choice_id_opt_break(x:&Option<ChoiceId>) -> (String,String) {
    if let Some(xx) = x {
        choice_id_break(xx)
    } else {
        (String::from("null"),String::from("null"))
    }
}
pub fn choice_id_break(x:&ChoiceId) -> (String,String) {
    match x {
        ChoiceId { choice_name, choice_owner } => {
            (
                format!("\"{}\"",choice_name),
                opt_py(&choice_owner, party_as_python)
            )
        }
    }
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

pub fn observation_box_as_python(x:&Box<Observation>) -> String {observation_as_python(x)}
pub fn observation_as_python(x:&Observation) -> String {
    match &x {
        Observation::AndObs { both, and} => 
            format!("Observation.AndObs({},{})", opt_py(both, observation_box_as_python), opt_py(and,observation_box_as_python)),
        Observation::OrObs { either, or} => 
            format!("Observation.OrObs({},{})", opt_py(either, observation_box_as_python), opt_py(or,observation_box_as_python)),
        Observation::NotObs { not} => 
            format!("Observation.NotObs({})", opt_py(not, observation_box_as_python)),
        Observation::ChoseSomething(x) => {
            let (choice_name,party) : (String,String) = choice_id_opt_break(x);
            format!("Observation.ChoseSomething(choice_name={choice_name}, party={party})" )
        },
        Observation::ValueGE { value, ge_than } => 
            format!("Observation.ValueGE({},{})", opt_py(value,value_box_as_python), opt_py(ge_than,value_box_as_python)),
        Observation::ValueGT { value, gt_than } => 
            format!("Observation.ValueGT({},{})", opt_py(value,value_box_as_python), opt_py(gt_than,value_box_as_python)),
        Observation::ValueLT { value, lt_than } => 
            format!("Observation.ValueLT({},{})", opt_py(value,value_box_as_python), opt_py(lt_than,value_box_as_python)),
        Observation::ValueLE { value, le_than } => 
            format!("Observation.ValueLE({},{})", opt_py(value,value_box_as_python), opt_py(le_than,value_box_as_python)),
        Observation::ValueEQ { value, equal_to} => 
            format!("Observation.ValueEQ({},{})", opt_py(value,value_box_as_python), opt_py(equal_to,value_box_as_python)),
        Observation::True => 
            "Observation.FalseObs()".into(),
        Observation::False => 
            "Observation.TrueObs()".into(),
    }
}

pub fn value_box_as_python(x:&Box<Value>) -> String {value_as_python(x)}
pub fn value_as_python(x:&Value) -> String {
    match &x {
        Value::AvailableMoney(party, token) => {
            let party_py = opt_py(party, party_as_python);
            let token_py = opt_py(token, token_as_python);
            format!("Value.AvailableMoney(account_of={party_py},token={token_py})")
            
        },
        Value::ConstantValue(x) => {
            format!("Value.Constant({x})")
        },
        Value::NegValue(x) => {
            format!("Value.NegValue({})",opt_py(x, value_box_as_python))
        },
        Value::AddValue(value_one, value_two) => {
            format!("Value.AddValue(value_one={},value_two={})",opt_py(value_one, value_box_as_python),opt_py(value_two, value_box_as_python))
        },
        Value::SubValue(a, b) => {
            format!("Value.SubValue(subtract={},from_val={})",opt_py(a, value_box_as_python),opt_py(b, value_box_as_python))
        },
        Value::MulValue(a, b) => {
            format!("Value.MulValue(multiply={},by={})",opt_py(a, value_box_as_python),opt_py(b, value_box_as_python))
        },
        Value::DivValue(a, b) => {
            format!("Value.DivValue(divide={},by={})",opt_py(a, value_box_as_python),opt_py(b, value_box_as_python))
        },
        Value::ChoiceValue(x) => {
            let (choice_name,choice_owner) = choice_id_opt_break(x);
            format!("Value.ChoiceValue(choice_name={choice_name},choice_owner={choice_owner})")
        },
        Value::TimeIntervalStart => "Value.TimeIntervalStart()".into(),
        Value::TimeIntervalEnd => "Value.TimeIntervalEnd()".into(),
        Value::UseValue(x) => {
            match x {
                ValueId::Name(name) => format!("Value.UseValue(\"{name}\")"),
            }
        },
        Value::Cond(r#if,value,r#else) => {
            let if_obs = opt_py(r#if, observation_as_python);
            let then_val = opt_py(value, value_box_as_python);
            let else_val =opt_py(r#else, value_box_as_python);
            format!("Value.Cond(if_obs={if_obs},then_val={then_val},else_val={else_val})")
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

pub fn case_as_python(x:&PossiblyMerkleizedCase) -> String {
    
    let (continue_with_py,case) = match x {
        PossiblyMerkleizedCase::Raw { case, then } => {
            let py_continuation = if let Some(c) = then {
                    format!("PossiblyMerkleizedContract.raw({})",contract_as_python(c))
            } else {
                    "null".into()
            };
            (py_continuation, case.clone())
        },
        PossiblyMerkleizedCase::Merkleized { case, then } => {
           let py_continuation = format!("PossiblyMerkleizedContract.merkleized(\"{then}\")");
           (py_continuation, Some(case.clone()))
        }
    };


    match case {
        Some(marlowe_lang::types::marlowe::Action::Notify { notify_if }) => {
            match &notify_if {
                Some(_obs) => {
                    let obs_py = opt_py(&notify_if, observation_as_python);
                    format!("Case.Notify(obs={obs_py},then_continue_with={continue_with_py})")
                },
                None => format!("Case.Notify(obs=null,then_continue_with={continue_with_py})"),
            }
        },
        Some(marlowe_lang::types::marlowe::Action::Deposit { into_account, party, of_token, deposits }) => {
            let into_account_py = opt_py(&into_account,party_as_python);
            let party_py = opt_py(&party,party_as_python);
            let of_token_py = opt_py(&of_token,token_as_python);
            let deposits_py = opt_py(&deposits,value_as_python);
            format!("Case.Deposit(into_account={into_account_py},by={party_py},of_token={of_token_py},value={deposits_py},then_continue_with={continue_with_py})")
        },
        Some(marlowe_lang::types::marlowe::Action::Choice { for_choice, choose_between }) => {
            
            let (choice_name_py,choice_owner_py) : (String,String) = match for_choice {
                Some(ChoiceId { choice_name, choice_owner }) => {
                    (
                        format!("\"{}\"",choice_name),
                        match choice_owner {
                            Some(o) => party_as_python(&o),
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
            format!("Contract.If(obs={if_py},then={then_py},else_contract={else_py})")
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
            format!("Contract.Let(variable_name=\"{var_name}\",be={be_py},then={then_py})")
        },
        marlowe_lang::types::marlowe::Contract::Assert { assert, then } => {
            let assert_py = opt_py(assert,observation_as_python);
            let then_py = opt_py(then,contract_box_as_python);
            format!("Contract.Assert(obs={assert_py},then={then_py})")
        },
    }
}
