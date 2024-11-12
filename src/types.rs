use std::collections::HashMap;

use pyo3::{
    exceptions::PyKeyError,
    prelude::*,
    types::{PyDict, PyList},
};
use serde::Deserialize;

#[derive(FromPyObject, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    String(String),
    Number(f64),
    Duration(std::time::Duration),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl ToPyObject for Value {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Value::Bool(b) => b.to_object(py),
            Value::String(s) => s.to_object(py),
            Value::Number(f) => f.to_object(py),
            Value::Duration(duration) => duration.to_object(py),
            Value::Array(vec) => {
                PyList::new_bound(py, vec.iter().map(|v| v.to_object(py))).to_object(py)
            }
            Value::Map(m) => {
                let result = PyDict::new_bound(py);
                for (k, v) in m.iter() {
                    result.set_item(k.to_object(py), v.to_object(py)).unwrap();
                }
                result.to_object(py)
            }
        }
    }
}

impl From<Value> for serde_json::Value {
    fn from(value: Value) -> Self {
        match value {
            Value::Bool(b) => serde_json::Value::Bool(b),
            Value::String(s) => serde_json::Value::String(s),
            Value::Number(f) => match serde_json::Number::from_f64(f) {
                Some(n) => serde_json::Value::Number(n),
                None => serde_json::Value::Null,
            },
            Value::Duration(duration) => (std::time::SystemTime::now() + duration)
                .duration_since(std::time::UNIX_EPOCH)
                .ok()
                .and_then(|d| serde_json::Number::from_f64(d.as_secs_f64()))
                .map(serde_json::Value::Number)
                .unwrap_or(serde_json::Value::Null),
            Value::Array(v) => serde_json::Value::Array(v.into_iter().map(|v| v.into()).collect()),
            Value::Map(m) => {
                let mut result = serde_json::Map::new();
                for (k, v) in m.into_iter() {
                    result.insert(k, v.into());
                }
                serde_json::Value::Object(result)
            }
        }
    }
}

#[pyclass]
pub struct TokenData {
    pub claims: HashMap<String, Value>,
}

#[pymethods]
impl TokenData {
    fn __getitem__(&self, py: Python<'_>, item: &str) -> PyResult<PyObject> {
        match self.claims.get(item) {
            Some(v) => Ok(v.to_object(py)),
            None => Err(PyKeyError::new_err("not found key {item}")),
        }
    }
}
