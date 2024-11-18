import time
from datetime import timedelta

import pytest

import rsjwt


@pytest.mark.parametrize(
    "alg",
    [
        "HS256",
        "HS384",
        "HS512",
    ],
)
def test_decode_dt(alg):
    v = rsjwt.JWT("123", algorithm=alg)
    data = {
        "exp": timedelta(seconds=10),
    }
    token = v.encode(data)
    td = v.decode(token)
    now = time.time()
    exp = td["exp"]
    assert isinstance(exp, float)
    assert now < exp < now + 10
