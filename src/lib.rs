use pyo3::prelude::*;
mod polymer;

use polymer::Point;
use polymer::Polymer;
use polymer::save_pairwise_distances;
// use polymer::save_conformers_as_h5;


/// Python module implemented in Rust.
#[pymodule]
fn brownian_walk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_class::<Polymer>()?;
    m.add_function(wrap_pyfunction!(save_pairwise_distances, m)?)?;
    // m.add_function(wrap_pyfunction!(save_conformers_as_h5, m)?);
    Ok(())
}

