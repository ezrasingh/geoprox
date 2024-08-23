# coding: utf-8

"""
    geoprox-server

    Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

    The version of the OpenAPI document: 0.5.0
    Contact: singhezra@gmail.com
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool, StrictFloat, StrictInt
from typing import Any, ClassVar, Dict, List, Optional, Union
from typing_extensions import Annotated
from typing import Optional, Set
from typing_extensions import Self

class QueryRange(BaseModel):
    """
    Arguments for range query
    """ # noqa: E501
    count: Optional[Annotated[int, Field(le=65535, strict=True, ge=1)]] = Field(default=None, description="Maximum number of neighbors that can be returned (default 100)")
    lat: Union[StrictFloat, StrictInt] = Field(description="Latitude")
    lng: Union[StrictFloat, StrictInt] = Field(description="Longitude")
    range: Annotated[int, Field(le=65535, strict=True, ge=0)] = Field(description="Search radius in kilometers")
    sorted: Optional[StrictBool] = Field(default=None, description="If enabled neighbors will be sorted by distance, nearest to furthest (default false)")
    __properties: ClassVar[List[str]] = ["count", "lat", "lng", "range", "sorted"]

    model_config = ConfigDict(
        populate_by_name=True,
        validate_assignment=True,
        protected_namespaces=(),
    )


    def to_str(self) -> str:
        """Returns the string representation of the model using alias"""
        return pprint.pformat(self.model_dump(by_alias=True))

    def to_json(self) -> str:
        """Returns the JSON representation of the model using alias"""
        # TODO: pydantic v2: use .model_dump_json(by_alias=True, exclude_unset=True) instead
        return json.dumps(self.to_dict())

    @classmethod
    def from_json(cls, json_str: str) -> Optional[Self]:
        """Create an instance of QueryRange from a JSON string"""
        return cls.from_dict(json.loads(json_str))

    def to_dict(self) -> Dict[str, Any]:
        """Return the dictionary representation of the model using alias.

        This has the following differences from calling pydantic's
        `self.model_dump(by_alias=True)`:

        * `None` is only added to the output dict for nullable fields that
          were set at model initialization. Other fields with value `None`
          are ignored.
        """
        excluded_fields: Set[str] = set([
        ])

        _dict = self.model_dump(
            by_alias=True,
            exclude=excluded_fields,
            exclude_none=True,
        )
        # set to None if count (nullable) is None
        # and model_fields_set contains the field
        if self.count is None and "count" in self.model_fields_set:
            _dict['count'] = None

        # set to None if sorted (nullable) is None
        # and model_fields_set contains the field
        if self.sorted is None and "sorted" in self.model_fields_set:
            _dict['sorted'] = None

        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of QueryRange from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "count": obj.get("count"),
            "lat": obj.get("lat"),
            "lng": obj.get("lng"),
            "range": obj.get("range"),
            "sorted": obj.get("sorted")
        })
        return _obj


