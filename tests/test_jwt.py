import time

import pytest

import rsjwt


def test_decode():
    v = rsjwt.JWT("123")
    data = {"exp": time.time() + 10}
    token = v.encode(data)
    td = v.decode(token)
    assert td["exp"] == data["exp"]
    
    
def test_decode_error():
    v = rsjwt.JWT("123")
    with pytest.raises(rsjwt.DecodeError):
        v.decode("random")
