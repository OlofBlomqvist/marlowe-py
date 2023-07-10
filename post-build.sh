#!/bin/bash

pip install maturin
maturin build --bindings pyo3 --release