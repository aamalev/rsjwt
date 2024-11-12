import time
from datetime import datetime, timedelta, timezone

import pytest

import rsjwt


def test_dir():
    d = [d for d in dir(rsjwt) if not d.startswith("_")]
    assert "JWT" in d


def test_decode():
    v = rsjwt.JWT("123")
    data: dict = {
        "exp": time.time() + 10,
        "s": "123",
        "a": ["123"],
        "m": {"a": 1},
    }
    token = v.encode(data)
    td = v.decode(token)
    assert td["exp"] == data["exp"]
    assert td["a"] == data["a"]
    assert td["s"] == data["s"]
    assert td["m"] == data["m"]


def test_decode_error():
    v = rsjwt.JWT("123")
    with pytest.raises(rsjwt.DecodeError):
        v.decode("random")


@pytest.mark.parametrize(
    "dt",
    [
        datetime.now(timezone.utc) + timedelta(seconds=5),
        time.time() + 5,
        timedelta(seconds=5),
    ],
)
def test_decode_dt(dt):
    v = rsjwt.JWT("123")
    data = {
        "exp": dt,
    }
    token = v.encode(data)
    td = v.decode(token)
    now = time.time()
    exp = td["exp"]
    assert isinstance(exp, float)
    assert now < exp < now + 10
