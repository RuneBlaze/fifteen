pub mod lca;
pub mod rmq;
use exposure::TreeSet;
use exposure::RMQ;
pub use lca::*;
pub mod exposure;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn table_five(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TreeSet>()?;
    m.add_class::<RMQ>()?;
    Ok(())
}
