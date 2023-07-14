## Requirements for building

Cargo/Rust, Maturin, Python 3.11+.

### Building

Windows: ./wheels.ps1
Linux & OSX: bash wheels.sh

### Using

```bash 
maturin develop
```

or

```bash
pip install .\target\wheels\marloweXXXXXX.whl --force-reinstall
```

### CI

The [build-wheels](https://github.com/OlofBlomqvist/marlowe-py/actions/workflows/wheely.yml) action is used for building the platform specific files which are packaged in the pymod package.


