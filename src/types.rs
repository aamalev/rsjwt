use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use pyo3::{exceptions::PyKeyError, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(FromPyObject, Deserialize, Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Value {
    Bool(bool),
    String(String),
    Int(i64),
    Float(f64),
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
            Value::List(v) => v.to_object(py),
            Value::Dict(m) => m.to_object(py),
        }
    }
}

#[pyclass]
#[derive(Debug)]
pub struct TokenData {
    #[pyo3(get)]
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

    fn get(&self, py: Python<'_>, item: &str) -> Option<PyObject> {
        self.claims.get(item).map(|v| v.to_object(py))
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.claims.len())
    }

    fn __contains__(&self, item: &str) -> PyResult<bool> {
        Ok(self.claims.contains_key(item))
    }

    fn keys(&self, py: Python<'_>) -> PyResult<PyObject> {
        let keys: Vec<String> = self.claims.keys().cloned().collect();
        Ok(keys.to_object(py))
    }

    fn values(&self, py: Python<'_>) -> PyResult<PyObject> {
        let values: Vec<PyObject> = self.claims.values().map(|v| v.to_object(py)).collect();
        Ok(values.to_object(py))
    }

    fn items(&self, py: Python<'_>) -> PyResult<PyObject> {
        let items: Vec<(String, PyObject)> = self
            .claims
            .iter()
            .map(|(k, v)| (k.clone(), v.to_object(py)))
            .collect();
        Ok(items.to_object(py))
    }

    fn __iter__(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.keys(py)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
