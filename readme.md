[![PyPI version](https://badge.fury.io/py/marlowe.svg)](https://pypi.org/project/marlowe/)

## MARLOWE-PY

**Python bindings for Marlowe_Lang [marlowe-rs](https://github.com/OlofBlomqvist/marlowe-rs).**

### Features

- Deserialize contracts from: cbor-hex, dsl, json.
- Deserialize datums & redeemers from cbor-hex,json.
- Serialize contracts to json, dsl.
- Serialize datums & redeemers to cbor-hex, json.

### Project goals

- Provide easy to use API's for everything marlowe-rs supports.
- Provide easy to use way of designing Marlowe contracts in Python.

### How to install

```bash
pip install marlowe
```

Or pip install wlh files directly:
- release versions: https://github.com/OlofBlomqvist/marlowe-py/releases
- latest builds artifact: https://github.com/OlofBlomqvist/marlowe-py/actions/workflows/wheely.yml


### How to use

See the example notebook [here](https://github.com/OlofBlomqvist/marlowe-py/blob/main/example.ipynb).


### Building from source

- See [development.md](https://github.com/OlofBlomqvist/marlowe-py/blob/main/development.md)


