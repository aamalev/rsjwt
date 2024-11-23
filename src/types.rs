use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use pyo3::{exceptions::PyKeyError, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(FromPyObject, IntoPyObject, Deserialize, Serialize, Clone, Debug)]
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

#[pyclass]
#[derive(Debug)]
pub struct TokenData {
    #[pyo3(get)]
    pub claims: HashMap<String, Value>,
}

#[pymethods]
impl TokenData {
    fn __getitem__(&self, item: &str) -> PyResult<Value> {
        self.claims
            .get(item)
            .cloned()
            .ok_or(PyKeyError::new_err("not found key {item}"))
    }

    fn get(&self, item: &str) -> Option<Value> {
        self.claims.get(item).cloned()
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.claims.len())
    }

    fn __contains__(&self, item: &str) -> PyResult<bool> {
        Ok(self.claims.contains_key(item))
    }

    fn keys(&self) -> PyResult<Vec<String>> {
        Ok(self.claims.keys().cloned().collect())
    }

    fn values(&self) -> PyResult<Vec<Value>> {
        Ok(self.claims.values().cloned().collect())
    }

    fn items(&self) -> PyResult<Vec<(String, Value)>> {
        Ok(self.claims.clone().into_iter().collect())
    }

    fn __iter__(&self) -> PyResult<Vec<String>> {
        self.keys()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self)
    }
}
