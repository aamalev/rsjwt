import time

import pytest

import rsjwt


def test_dir():
    d = [d for d in dir(rsjwt) if not d.startswith("_")]
    assert "JWT" in d


def test_decode():
    v = rsjwt.JWT("123")
    data = {
        "exp": time.time() + 10,
        "s": "123",
        "a": ["123", 123],
    }
    token = v.encode(data)
    td = v.decode(token)
    assert td["exp"] == data["exp"]
    assert td["a"] == data["a"]
    assert td["s"] == data["s"]
    
    
def test_decode_error():
    v = rsjwt.JWT("123")
    with pytest.raises(rsjwt.DecodeError):
        v.decode("random")
