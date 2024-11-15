import argparse
import json
import sys
from dataclasses import dataclass
from timeit import timeit
from typing import Callable

ALG = "HS256"
HEADER = {"alg": ALG}
DATA = {"some": "payload"}
SECRET = "secret"


@dataclass
class Item:
    name: str
    decode: Callable


def on_pyjwt() -> Item:
    import jwt

    token = jwt.encode(DATA, SECRET, algorithm=ALG)
    item = Item(
        name="pyjwt",
        decode=lambda: jwt.decode(token, SECRET, algorithms=[ALG]),
    )
    assert DATA == item.decode()
    return item


def on_jose() -> Item:
    from jose import jwt

    token = jwt.encode(DATA, SECRET, algorithm=ALG)
    item = Item(
        name="python-jose",
        decode=lambda: jwt.decode(token, SECRET, algorithms=[ALG]),
    )
    assert DATA == item.decode()
    return item


def on_authlib() -> Item:
    from authlib.jose import jwt

    token = jwt.encode(HEADER, DATA, SECRET)
    item = Item(
        name="authlib",
        decode=lambda: jwt.decode(token, SECRET),
    )
    assert DATA == item.decode()
    return item


def on_jwcrypto() -> Item:
    from jwcrypto import jwk, jwt

    t = jwt.JWT(header=HEADER, claims=DATA)
    key = jwk.JWK.from_password(SECRET)
    t.make_signed_token(key)
    token = t.serialize()

    def decode():
        t.deserialize(token, key)
        return t.claims

    item = Item(
        name="jwcrypto",
        decode=decode,
    )
    decoded = json.loads(item.decode())
    assert DATA == decoded, decoded
    return item


def on_rsjwt() -> Item:
    import rsjwt

    c = rsjwt.JWT(SECRET, required_spec_claims=[])

    token = c.encode(DATA)
    item = Item(
        name="rsjwt",
        decode=lambda: c.decode(token),
    )
    td = item.decode()
    for k in DATA:
        assert DATA[k] == td[k]
    return item


def main(opts: argparse.Namespace):
    print("Python:", sys.version)
    print("Algorithm:", ALG)
    print("Iterations:", opts.n)
    print()
    c1, c2, c3 = 15, 15, 15
    print(
        "|",
        "package".rjust(c1),
        "|",
        "secs".center(c2),
        "|",
        "n".center(c3),
        "|",
    )
    print(
        "|",
        "-" * c1,
        "|",
        "-" * c2,
        "|",
        "-" * c3,
        "|",
    )
    base = None

    for f in (
        on_rsjwt,
        on_pyjwt,
        on_authlib,
        on_jose,
        on_jwcrypto,
    ):
        try:
            item = f()
        except Exception:
            continue
        decode_time = timeit(item.decode, number=opts.n)
        if not base:
            base = decode_time
        print(
            "|",
            item.name.rjust(c1),
            "|",
            f"{decode_time:.4f}".rjust(c2),
            "|",
            f"{decode_time / base:.3f}".rjust(c3),
            "|",
        )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", type=int, default=1000000)
    main(parser.parse_args())
