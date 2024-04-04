## POC

1. create and activate venv
    1. virtualenv .venv
    2. linux: source .venv/bin/activate
    3. windows: .venv\Scripts\activate
2. pip install -e my_python_lib maturin
3. maturin b -m crates/core/Cargo.toml
4. pip install target/wheels/*.whl
5. python main.py
