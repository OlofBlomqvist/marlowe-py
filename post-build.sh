#!/bin/bash

apt-install python3
pip install maturin
python maturin build --bindings pyo3