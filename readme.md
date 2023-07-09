## Marlowe-RS-PY

The Python module for Marlowe-RS

---


To compile your PyO3 project into a Python module that can be installed with pip, you can use the `maturin` tool. Here are the steps to follow:

1. **Navigate to your project root directory**. The root directory of your project is the one that contains `Cargo.toml`.

2. **Build your project with `maturin`**. Run the following command:
// python C:\Users\Oblink\AppData\Local\packages\pythonsoftwarefoundation.python.3.11_qbz5n2kfra8p0\localcache\local-packages\python311\site-packages\maturin

winget install -e --id Microsoft.VisualStudio.2022.BuildTools

https://visualstudio.microsoft.com/visual-cpp-build-tools/

https://aka.ms/vs/17/release/vs_BuildTools.exe


// apt install gcc ?


 rustup target add x86_64-pc-windows-gnu 
 rustup target add x86_64-apple-darwin
 rustup target add x86_64-unknown-linux-gnu
 
 cargo install cc // for darwin

// win build
maturin build --target x86_64-pc-windows-msvc --bindings pyo3 --release 

// osx build
maturin build --target x86_64-apple-darwin --bindings pyo3 --release

// lin build
maturin build --target x86_64-unknown-linux-gnu --bindings pyo3 --release

// pip inst
pip install C:\users\oblink\documents\github\marlowe_rust\marlowepy\target\wheels\marlowepy-0.1.0-cp311-none-win_amd64.whl --force-reinstall
---

   ```shell
   maturin build --release
   ```

   The `--release` flag means to build in release mode, which includes optimizations and may take a bit longer than a debug build. If you want to create a quick-and-dirty build for testing, you can leave out this flag.

3. **Find the output wheel file**. The `maturin build` command will output a Python wheel file, which is a type of distribution format for Python packages. The wheel file will be in the `target/wheels` directory in your project root directory. The filename will be something like `my_project-0.1.0-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl`, but the exact filename will depend on your project name, version, and the Python version and platform you're building for.

4. **Install the module with pip**. You can install the wheel file with pip just like any other Python package. For example:

   ```shell
   pip install target/wheels/my_project-0.1.0-cp39-cp39-manylinux_2_5_x86_64.manylinux1_x86_64.whl
   ```

   Once the module is installed, you can import it in Python using the project name you specified when you ran `maturin init`.

5. **Distribute the module**. If you want to distribute your module so that others can install it, you can upload the wheel file to the Python Package Index (PyPI) using a tool like `twine`. First, you'll need to create a PyPI account if you don't already have one. Then, you can upload your project with the following command:

   ```shell
   twine upload target/wheels/*.whl
   ```

   After running this command, your project will be available for anyone to install with `pip install my_project` (replacing `my_project` with your project name).

Remember that building a Python module with PyO3 and `maturin` requires a Rust toolchain and Python development headers to be installed on your machine. If you're missing any dependencies, the `maturin build` command may fail. If you run into any issues, make sure you have all the necessary dependencies installed and try again.



---

dev console: maturin develop
more info: https://pyo3.rs/main/
