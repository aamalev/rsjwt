mod error;
mod types;
use error::{DecodeError, EncodeError};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pyo3::{prelude::*, types::PyModule, PyResult, Python};
use std::{collections::HashMap, str::FromStr};
use types::{TokenData, Value};

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
    #[pyo3(signature = (
        secret,
        required_spec_claims=None,
        algorithm="HS256",
    ))]
    fn new(secret: String, required_spec_claims: Option<Vec<String>>, algorithm: &str) -> Self {
        let algorithm = Algorithm::from_str(algorithm).unwrap_or_default();
        let mut validation = Validation::new(algorithm);
        if let Some(ref r) = required_spec_claims {
            validation.set_required_spec_claims(r);
        }
        let header = Header::new(algorithm);
        Self {
            header,
            key: EncodingKey::from_secret(secret.as_ref()),
            validation,
            secrets: vec![DecodingKey::from_secret(secret.as_ref())],
        }
    }

    fn encode(&self, claims: HashMap<String, Value>) -> PyResult<String> {
        let claims = Value::Dict(claims);
        encode(&self.header, &claims, &self.key).map_err(|_| EncodeError::new_err("invalid claims"))
    }

    fn decode(&self, token: String) -> PyResult<TokenData> {
        let mut result = Err(DecodeError::new_err("not valid token"));
        for secret in self.secrets.iter() {
            match decode::<Value>(&token, secret, &self.validation) {
                Ok(jsonwebtoken::TokenData {
                    header: _,
                    claims: Value::Dict(claims),
                }) => {
                    result = Ok(TokenData { claims });
                    break;
                }
                Err(e) => result = Err(DecodeError::new_err(e.to_string())),
                _ => (),
            }
        }
        result
    }
}

#[pymodule]
fn rsjwt(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("EncodeError", py.get_type::<EncodeError>())?;
    m.add("DecodeError", py.get_type::<DecodeError>())?;
    m.add_class::<JWT>()?;
    m.add_class::<TokenData>()?;
    Ok(())
}
