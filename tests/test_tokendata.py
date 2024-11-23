import time

import pytest

import rsjwt


def test_mapping():
    v = rsjwt.JWT("123")
    data: dict = {
        "exp": int(time.time()) + 10,
    }
    token = v.encode(data)
    td = v.decode(token)
    assert dict(td) == data


def test_mapping_eq():
    v = rsjwt.JWT("123")
    data: dict = {
        "exp": int(time.time()) + 10,
    }
    token = v.encode(data)
    td = v.decode(token)
    assert td.claims == data


def test_mapping_getitem():
    v = rsjwt.JWT("123", required_spec_claims=[])
    data: dict = {
        "s": "123",
    }
    token = v.encode(data)
    td = v.decode(token)
    assert td["s"] == data["s"]

    with pytest.raises(KeyError):
        assert td["x"]


def test_mapping_get():
    v = rsjwt.JWT("123", required_spec_claims=[])
    data: dict = {
        "s": "123",
    }
    token = v.encode(data)
    td = v.decode(token)
    assert td.get("s") == data["s"]
    assert td.get("x") is None
