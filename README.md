# Connect pandas, numpy or polars with Rust.

1. Create a virtual environment with python3.9 and install the dependencies.
2. Run in terminal `maturin develop --release`.
3. Run `main.py` for see your times.

To run the modules made in Rust you must use Maturin.

These were the times in MacOs M1:

- Time Python + Pandas: 219.1022469997406
- Time Rust + Pandas: 4.293076276779175
- Time Rust + Polars: 1.128321886062622
