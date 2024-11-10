use pyo3::prelude::*;

#[derive(FromPyObject, Clone, Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Array(Vec<String>),
}
