#!/bin/bash
# Triggered by github CI for osx and linux builds

pip install maturin
maturin build --bindings pyo3 --release