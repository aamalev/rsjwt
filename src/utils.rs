use std::collections::HashMap;

use crate::types::Value;
use pyo3::{
    prelude::*,
    types::{PyDict, PyList},
    Python,
};

fn _to_json(v: Value) -> serde_json::Value {
    match v {
        Value::String(s) => serde_json::Value::String(s),
        Value::Number(f) => match serde_json::Number::from_f64(f) {
            Some(n) => serde_json::Value::Number(n),
            None => serde_json::Value::Null,
        },
        Value::Array(v) => {
            serde_json::Value::Array(v.into_iter().map(serde_json::Value::String).collect())
        }
    }
}

pub fn to_json(v: HashMap<String, Value>) -> PyResult<serde_json::Value> {
    let mut result = serde_json::Map::new();
    for (k, v) in v.into_iter() {
        let v = _to_json(v);
        result.insert(k, v);
    }
    Ok(serde_json::Value::Object(result))
}

pub fn from_json(py: Python<'_>, v: &serde_json::Value) -> PyResult<PyObject> {
    Ok(match v {
        serde_json::Value::Null => py.None(),
        serde_json::Value::Bool(b) => b.to_object(py),
        serde_json::Value::Number(n) => {
            if n.is_f64() {
                n.as_f64().to_object(py)
            } else if n.is_i64() {
                n.as_i64().to_object(py)
            } else {
                n.as_u64().to_object(py)
            }
        }
        serde_json::Value::String(s) => s.to_object(py),
        serde_json::Value::Array(a) => {
            let mut result = vec![];
            for v in a.iter() {
                result.push(from_json(py, v)?);
            }
            PyList::new_bound(py, result).to_object(py)
        }
        serde_json::Value::Object(m) => {
            let result = PyDict::new_bound(py);
            for (k, v) in m.into_iter() {
                result.set_item(k, from_json(py, v)?).unwrap();
            }
            result.to_object(py)
        }
    })
}
