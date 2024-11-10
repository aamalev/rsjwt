use pyo3::create_exception;

create_exception!(exceptions, DecodeError, pyo3::exceptions::PyException);
create_exception!(exceptions, EncodeError, pyo3::exceptions::PyException);
