$f = get-item .\target\wheels\*.whl | select -first 1 | select -ExpandProperty fullname
pip install "$f" --force-reinstall