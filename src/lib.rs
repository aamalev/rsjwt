mod error;
mod types;
mod utils;
use error::{DecodeError, EncodeError};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use pyo3::{exceptions::PyKeyError, prelude::*, types::PyModule, PyResult, Python};
use std::collections::HashMap;
use types::Value;
use utils::{from_json, to_json};

#[pyclass]
struct TokenData {
    claims: serde_json::Value,
}

#[pymethods]
impl TokenData {
    fn __getitem__(&self, py: Python<'_>, item: &str) -> PyResult<PyObject> {
        match self.claims.get(item) {
            Some(v) => from_json(py, v),
            None => Err(PyKeyError::new_err("not found key {item}")),
        }
    }
}

#[pyclass]
#[allow(clippy::upper_case_acronyms)]
struct JWT {
    header: Header,
    key: EncodingKey,
    validation: Validation,
    secrets: Vec<DecodingKey>,
}

#[pymethods]
impl JWT {
    #[new]
    #[pyo3(signature = (secret, required_spec_claims=None))]
    fn new(secret: String, required_spec_claims: Option<Vec<String>>) -> Self {
        let mut validation = Validation::default();
        if let Some(ref r) = required_spec_claims {
            validation.set_required_spec_claims(r);
        }
        Self {
            header: Header::default(),
            key: EncodingKey::from_secret(secret.as_ref()),
            validation,
            secrets: vec![DecodingKey::from_secret(secret.as_ref())],
        }
    }

    fn encode(&self, claims: HashMap<String, Value>) -> PyResult<String> {
        let claims = to_json(claims)?;
        encode(&self.header, &claims, &self.key).map_err(|_| EncodeError::new_err("invalid claims"))
    }

    fn decode(&self, token: String) -> PyResult<TokenData> {
        let mut result = Err(DecodeError::new_err("not valid token"));
        for secret in self.secrets.iter() {
            match decode::<serde_json::Value>(&token, secret, &self.validation) {
                Ok(token) => {
                    let claims = token.claims;
                    result = Ok(TokenData { claims });
                    break;
                }
                Err(e) => result = Err(DecodeError::new_err(e.to_string())),
            }
        }
        result
    }
}

#[pymodule]
fn rsjwt(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("EncodeError", py.get_type_bound::<EncodeError>())?;
    m.add("DecodeError", py.get_type_bound::<DecodeError>())?;
    m.add_class::<JWT>()?;
    m.add_class::<TokenData>()?;
    Ok(())
}
