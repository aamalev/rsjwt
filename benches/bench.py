import argparse
import json
import sys
from timeit import timeit
from typing import Callable, List, Optional

import rsjwt

ALG = "HS256"
HEADER = {"alg": ALG}
DATA = {"some": "payload"}
SECRET = "secret"


class Item:
    name: str
    decode: Optional[Callable] = None


class ItemRsJWT(Item):
    name = "rsjwt"

    def __init__(self):
        import rsjwt

        c = rsjwt.JWT(SECRET, required_spec_claims=[])
        # token = c.encode(DATA)
        self.decode = lambda token: c.decode(token).claims


class ItemPyJWT(Item):
    name = "pyjwt"

    def __init__(self):
        import jwt

        # token = jwt.encode(DATA, SECRET, algorithm=ALG)
        self.decode = lambda token: jwt.decode(token, SECRET, algorithms=[ALG])


class ItemAuthLib(Item):
    name = "authlib"

    def __init__(self):
        from authlib.jose import jwt

        # token = jwt.encode(HEADER, DATA, SECRET)
        self.decode = lambda token: jwt.decode(token, SECRET)


class ItemJose(Item):
    name = "python-jose"

    def __init__(self):
        try:
            from jose import jwt
        except Exception:
            pass
        else:
            # token = jwt.encode(DATA, SECRET, algorithm=ALG)
            self.decode = lambda token: jwt.decode(token, SECRET, algorithms=[ALG])


class ItemJwCripto(Item):
    name = "jwcrypto"

    def __init__(self):
        from jwcrypto import jwk, jwt

        key = jwk.JWK.from_password(SECRET)

        t = jwt.JWT(header=HEADER)
        # t.make_signed_token(key)
        # token = t.serialize()

        def decode(token):
            t.deserialize(token, key)
            return json.loads(t.claims)

        self.decode = decode


class Table:
    sep = "|"

    def __init__(self, colums: List[str], default_width=12):
        self.colums = colums
        self._default = default_width

    def print_line(self, *args: str):
        line = self.sep
        for item in args:
            line += item.rjust(self._default)
            line += self.sep
        print(line)

    def print_head(self):
        self.print_line(*self.colums)
        args = []
        for _ in self.colums:
            args.append("-" * self._default)
        self.print_line(*args)


def main(opts: argparse.Namespace):
    print("Python:", sys.version)
    print("Iterations:", opts.n)
    print()
    print("Algorithm:", ALG)
    print("Method: decode")
    table = Table(
        colums=[
            "package",
            "secs",
            "n",
        ]
    )
    table.print_head()

    base = None

    token = rsjwt.JWT(SECRET).encode(DATA)

    for f in Item.__subclasses__():
        item = f()
        if item.decode is not None:
            assert item.decode(token) == DATA

            decode_time = timeit(lambda: item.decode(token), number=opts.n)  # noqa
            if not base:
                base = decode_time
            table.print_line(
                item.name,
                f"{decode_time:.4f}",
                f"{decode_time / base:.3f}",
            )
        else:
            table.print_line(
                item.name,
                "fail",
                "",
            )


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-n", type=int, default=1000000)
    main(parser.parse_args())
