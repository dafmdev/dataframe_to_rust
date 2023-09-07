mod is_prime_mod;

use pyo3::prelude::*;
use rayon::prelude::*;
use pyo3_polars::PyDataFrame;
use pyo3_polars::error::PyPolarsErr;
use polars::prelude::*;



#[pyfunction]
fn is_prime(values: Vec<i64>) -> Vec<bool> {
    values.into_par_iter().map(is_prime_mod::is_prime_scalar).collect()
}

#[pyfunction]
fn is_prime_pl(pydf: PyDataFrame, col_a: &str) -> PyResult<PyDataFrame> {
    let df: DataFrame = pydf.into();
    let df = is_prime_mod::is_prime_scalar_polars(df, col_a).map_err(PyPolarsErr::from)?;
    Ok(PyDataFrame(df))
}

/// A Python module implemented in Rust.
#[pymodule]
fn counts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    m.add_function(wrap_pyfunction!(is_prime_pl, m)?)?;
    Ok(())
}