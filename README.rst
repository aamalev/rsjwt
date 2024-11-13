rsjwt
=====

.. image:: https://img.shields.io/badge/License-MIT-blue.svg
   :target: https://lbesson.mit-license.org/

.. image:: https://img.shields.io/pypi/v/rsjwt.svg
  :target: https://pypi.org/project/rsjwt

.. image:: https://img.shields.io/pypi/pyversions/rsjwt.svg
  :target: https://pypi.org/project/rsjwt
  :alt: Python versions

.. image:: https://readthedocs.org/projects/rsjwt/badge/?version=latest
  :target: https://github.com/aamalev/rsjwt#rsjwt
  :alt: Documentation Status

.. image:: https://github.com/aamalev/rsjwt/workflows/Tests/badge.svg
  :target: https://github.com/aamalev/rsjwt/actions?query=workflow%3ATests

.. image:: https://img.shields.io/pypi/dm/rsjwt.svg
  :target: https://pypi.org/project/rsjwt

|

.. image:: https://img.shields.io/badge/Rustc-1.80.0-blue?logo=rust
  :target: https://www.rust-lang.org/

.. image:: https://img.shields.io/badge/cargo-clippy-blue?logo=rust
  :target: https://doc.rust-lang.org/stable/clippy/

.. image:: https://img.shields.io/badge/PyO3-maturin-blue.svg
  :target: https://github.com/PyO3/maturin

.. image:: https://img.shields.io/badge/PyO3-asyncio-blue.svg
  :target: https://github.com/awestlake87/pyo3-asyncio

.. image:: https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/astral-sh/ruff/main/assets/badge/v2.json
  :target: https://github.com/astral-sh/ruff
  :alt: Linter: ruff

.. image:: https://img.shields.io/badge/code%20style-ruff-000000.svg
  :target: https://github.com/astral-sh/ruff
  :alt: Code style: ruff

.. image:: https://img.shields.io/badge/types-Mypy-blue.svg
  :target: https://github.com/python/mypy
  :alt: Code style: Mypy

.. image:: https://img.shields.io/badge/%F0%9F%A5%9A-Hatch-4051b5.svg
  :alt: Hatch project
  :target: https://github.com/pypa/hatch


Python wrapper for:
  | `jsonwebtoken <https://github.com/Keats/jsonwebtoken>`_,


Features
--------

* Encode and decode JWT


Install
-------

.. code-block:: shell

    pip install rsjwt


Using
-----

.. code-block:: python

  from datetime import timedelta

  import rsjwt


  v = rsjwt.JWT("123")
  data = {
      "exp": timedelta(hours=8),
      "s": "123",
      "a": ["123", 123],
      "m": {"a": 1},
  }
  token = v.encode(data)
  assert isinstance(token, str)

  td = v.decode(token)
  assert td["a"] == data["a"]
  assert td["s"] == data["s"]
  assert td["m"] == data["m"]
  assert isinstance(td["exp"], float)


Bench
-----

.. code-block:: bash

    % hatch run bench:py
    Python: 3.13.0 (main, Oct  7 2024, 05:02:14) [Clang 16.0.0 (clang-1600.0.26.4)]
    Algorithm: HS256
    Iterations: 1000000

    |         package |       secs      |        n        |
    | --------------- | --------------- | --------------- |
    |           rsjwt |      2.3461     |      1.000      |
    |           pyjwt |      6.3386     |      2.702      |
    |         authlib |      7.4025     |      3.155      |


Development
-----------

.. code-block:: bash

    cargo fmt
    cargo clippy
    maturin develop


or use hatch envs:

.. code-block:: bash

    hatch run fmt
    hatch run check
    hatch run build
