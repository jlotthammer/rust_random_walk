use pyo3::prelude::*;
use rand::prelude::*;
use rayon::prelude::*;
// use std::time::Instant;

/// Rust implementation of the `Point` struct.
#[pyclass]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[pymethods]
impl Point {
    #[new]
    fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }

    #[getter]
    fn x(&self) -> f32 {
        self.x
    }

    #[getter]
    fn y(&self) -> f32 {
        self.y
    }

    #[getter]
    fn z(&self) -> f32 {
        self.z
    }
}


/// Rust implementation of the `random_walk` function.
#[pyfunction]
fn random_walk(num_steps: usize, step_size: f32) -> Point {
    let mut rng = rand::thread_rng();
    let mut current: Point = Point::new(0.0, 0.0, 0.0);

    for _ in 0..num_steps {
        let theta = rng.gen::<f32>() * 2.0 * std::f32::consts::PI;
        let phi = rng.gen::<f32>() * std::f32::consts::PI;
        let dx: f32 = step_size * theta.cos() * phi.sin();
        let dy: f32 = step_size * theta.sin() * phi.sin();
        let dz: f32 = step_size * phi.cos();

        current = Point::new(
            current.x + dx as f32,
            current.y + dy as f32,
            current.z + dz as f32,
        );
    }

    current
}

/// Rust implementation of the `parallel_mean_squared_displacement` function.
#[pyfunction]
fn parallel_mean_squared_displacement(k: usize, n: usize, l: f32) -> f32 {
    let total_msd: f32 = (0..k).into_par_iter().map(|_| {
        let final_point = random_walk(n, l);
        let dx: f32 = final_point.x;
        let dy: f32 = final_point.y;
        let dz: f32 = final_point.z;
        dx * dx + dy * dy + dz * dz
    }).sum();

    total_msd / k as f32
}

/// Python module implemented in Rust.
#[pymodule]
fn brownian_walk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Point>()?;
    m.add_function(wrap_pyfunction!(random_walk, m)?)?;
    m.add_function(wrap_pyfunction!(parallel_mean_squared_displacement, m)?)?;
    Ok(())
}

