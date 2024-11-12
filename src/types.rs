use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use pyo3::{
    exceptions::PyKeyError,
    prelude::*,
    types::{PyDict, PyList},
};
use serde::{Deserialize, Serialize};

#[derive(FromPyObject, Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
    #[serde(serialize_with = "serialize_timedelta")]
    TimeDelta(Duration),
    #[serde(serialize_with = "serialize_datetime")]
    DateTime(SystemTime),
    List(Vec<Value>),
    Dict(HashMap<String, Value>),
}

fn serialize_timedelta<S>(d: &Duration, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let dt = SystemTime::now() + *d;
    s.serialize_f64(to_f64(&dt))
}

fn serialize_datetime<S>(dt: &SystemTime, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_f64(to_f64(dt))
}

fn to_f64(dt: &SystemTime) -> f64 {
    dt.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .map(|d| d.as_secs_f64())
        .unwrap_or(0.0)
}

impl ToPyObject for Value {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Value::Bool(b) => b.to_object(py),
            Value::String(s) => s.to_object(py),
            Value::Float(f) => f.to_object(py),
            Value::Int(i) => i.to_object(py),
            Value::TimeDelta(duration) => duration.to_object(py),
            Value::DateTime(system_time) => system_time.to_object(py),
            Value::List(vec) => {
                PyList::new_bound(py, vec.iter().map(|v| v.to_object(py))).to_object(py)
            }
            Value::Dict(m) => m
                .iter()
                .fold(PyDict::new_bound(py), |d, (k, v)| {
                    d.set_item(k.to_object(py), v.to_object(py))
                        .unwrap_or_default();
                    d
                })
                .to_object(py),
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
