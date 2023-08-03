[![PyPI version](https://badge.fury.io/py/marlowe.svg)](https://pypi.org/project/marlowe/)
[![Downloads](https://static.pepy.tech/badge/marlowe)](https://pepy.tech/project/marlowe)



## MARLOWE-PY

**Python bindings for Marlowe_Lang [marlowe-rs](https://github.com/OlofBlomqvist/marlowe-rs).**


### Features

- Deserialize contracts from: cbor-hex, dsl, json.
- Deserialize datums & redeemers from cbor-hex,json.
- Serialize contracts to json, dsl.
- Serialize datums & redeemers to cbor-hex, json.
- Unstable: View contract expected next inputs and other state info.
- Unstable: Define contracts directly in Python.
- Unstable: Code generator: Convert any contract to equivalent Python code


### Project goals

- Provide easy to use API's for everything marlowe-rs supports.
- Provide easy to use way of designing Marlowe contracts in Python.

### How to install

```bash
pip install marlowe
```

Alternatives:
- pip install wlh files directly using builds artifacts from CI: https://github.com/OlofBlomqvist/marlowe-py/actions/workflows/build_wheels.yml
- clone the repo, install prereqs: rust,python,maturin; run wheels.ps1 or wheels.sh

### How to use

See the example notebook [here](https://github.com/OlofBlomqvist/marlowe-py/blob/main/example.ipynb).

Basic contract construction:
```python 
When(
    case=[
        Deposit(
            by = Role("GUY1"),
            value = Constant(42000000), 
            of_token = ADA, 
            into_account = Role("GUY2"),
            then_continue_with = raw(Close()) # or merkleized("some hash")
        )
    ], 
    timeout = TimeConst(999999999), 
    timeout_continuation = Close()
)
```

### Building from source

- See [development.md](https://github.com/OlofBlomqvist/marlowe-py/blob/main/development.md)


