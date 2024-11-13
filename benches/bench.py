from timeit import timeit

DATA = {"some": "payload"}
SECRET = "secret"

N = 1000000


def on_pyjwt():
    import jwt

    token = jwt.encode(DATA, SECRET, algorithm="HS256")

    decode_time = timeit(lambda: jwt.decode(token, SECRET, algorithms=["HS256"]), number=N)

    print(f"PyJWT: Decode time ({N} iterations): {decode_time:.4f} seconds")


def on_jose():
    from jose import jwt

    token = jwt.encode(DATA, SECRET, algorithm="HS256")

    decode_time = timeit(lambda: jwt.decode(token, SECRET, algorithms=["HS256"]), number=N)

    print(f"Python-Jose: Decode time ({N} iterations): {decode_time:.4f} seconds")


def on_rsjwt():
    import rsjwt

    c = rsjwt.JWT(SECRET, required_spec_claims=[])

    token = c.encode(DATA)

    decode_time = timeit(lambda: c.decode(token)["some"], number=N)

    print(f"RSJWT: Decode time ({N} iterations): {decode_time:.4f} seconds")


if __name__ == "__main__":
    on_pyjwt()
    # on_jose()  # fail on py3.13
    on_rsjwt()
