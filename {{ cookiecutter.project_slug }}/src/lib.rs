#![feature(proc_macro, specialization)]

pub extern crate pyo3;

use pyo3::prelude::*;
use pyo3::py::modinit as pymodinit;

#[cfg(test)]
mod tests;

fn add(a: f64, b: f64) -> f64 {
    a + b
}


#[pymodinit(example)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {

    #[pyfn(m, "add")]
    fn add_py(a: f64, b:f64) -> PyResult<f64> {
        add(a, b)
    }

    Ok(())
}

