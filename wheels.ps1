# Triggered for the windows build in github ci

pip install maturin
maturin build --bindings pyo3 --release

# pip install .\target\wheels\marlowe-0.1.0-cp311-none-win_amd64.whl