use pyo3::prelude::*;
use rayon::prelude::*;

fn is_prime_scalar(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }
    true
}

#[pyfunction]
fn is_prime(values: Vec<u32>) -> Vec<bool> {
    //values.into_iter().map(counts_5s).collect()
    values.into_par_iter().map(is_prime_scalar).collect()
}

/// A Python module implemented in Rust.
#[pymodule]
fn counts(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_prime, m)?)?;
    Ok(())
}