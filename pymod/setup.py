from setuptools import setup

setup(
    name="marlowe_lang",
    version="0.1.0",
    packages=[],  # Include an empty package
    py_modules=["marlowe_lang"],
    package_data={"marlowepy": ["marlowepy-0.1.0-cp311-none-win_amd64.whl"]},  # Include the wheel file
    install_requires=["wheel"],  # Add any required dependencies
)