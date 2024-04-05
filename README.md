## POC

1. create and activate venv
    1. virtualenv .venv
    2. linux: source .venv/bin/activate
    3. windows: .venv\Scripts\activate
2. pip install -e my_python_lib maturin
3. maturin b -m crates/core/Cargo.toml
4. pip install target/wheels/*.whl
5. python main.py

if you want to deploy the resulting executable, that can be done with pyinstaller

6. pip install pyinstaller
7. pyinstaller main.py --hidden-import my_python_lib --hidden-import my_python_lib.lib
8. ./dist/main/main

