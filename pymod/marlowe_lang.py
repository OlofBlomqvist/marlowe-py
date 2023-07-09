import platform
import pkg_resources
import subprocess
import sys

def is_package_installed(package_name, version):
    try:
        pkg_resources.get_distribution(package_name).version == version
        return True
    except pkg_resources.DistributionNotFound:
        return False

def install_marlowepy_wheel():
    package_name = "marlowepy"
    desired_version = "0.1.0"
    if is_package_installed(package_name, desired_version):
        return
    system = platform.system()
    if system == "Windows":
        wheel_file = "marlowepy-0.1.0-cp311-none-win_amd64.whl"
    elif system == "Linux":
        wheel_file = "marlowepy-0.1.0-linux.whl"
    else:
        raise NotImplementedError("Unsupported operating system")
    wheel_path = pkg_resources.resource_filename(__name__, wheel_file)
    subprocess.check_call([sys.executable, "-m", "pip", "install", wheel_path])

install_marlowepy_wheel()


# -------------------------------------------------------------------------------------------------------

import marlowepy

print(marlowepy.example())

def my_function():
    
    pass

def my_other_function():
    # Your code here
    pass


__all__ = ["example"]