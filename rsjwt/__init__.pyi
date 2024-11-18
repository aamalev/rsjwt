__all__ = [
    "DecodeError",
    "EncodeError",
    "JWT",
]

from datetime import datetime, timedelta
from typing import List, Literal, Mapping, Optional, Union

from .exceptions import DecodeError, EncodeError

SYMMETRIC = Literal[
    "HS256",
    "HS384",
    "HS512",
]
Value = Union[str, int, float, List[Value], Mapping[str, Value], timedelta, datetime]
TokenData = Mapping[str, Value]

class JWT:
    def __init__(
        self,
        secret: str,
        *,
        required_spec_claims: Optional[List[str]] = None,
        algorithm: SYMMETRIC = "HS256",
    ): ...
    def encode(self, claims: Mapping[str, Value]) -> str: ...
    def decode(self, token: str) -> TokenData: ...
