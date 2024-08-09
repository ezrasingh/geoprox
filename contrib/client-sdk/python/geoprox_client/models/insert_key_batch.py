# coding: utf-8

"""
    geoprox-server

    Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

    The version of the OpenAPI document: 0.4.1
    Contact: singhezra@gmail.com
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


from __future__ import annotations
import pprint
import re  # noqa: F401
import json

from pydantic import BaseModel, ConfigDict, Field, StrictBool
from typing import Any, ClassVar, Dict, List
from geoprox_client.models.insert_key import InsertKey
from typing import Optional, Set
from typing_extensions import Self

class InsertKeyBatch(BaseModel):
    """
    Arguments for inserting multiple keys
    """ # noqa: E501
    keys: List[InsertKey] = Field(description="Object key")
    preserve_order: StrictBool
    __properties: ClassVar[List[str]] = ["keys", "preserve_order"]

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
        """Create an instance of InsertKeyBatch from a JSON string"""
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
        # override the default output from pydantic by calling `to_dict()` of each item in keys (list)
        _items = []
        if self.keys:
            for _item in self.keys:
                if _item:
                    _items.append(_item.to_dict())
            _dict['keys'] = _items
        return _dict

    @classmethod
    def from_dict(cls, obj: Optional[Dict[str, Any]]) -> Optional[Self]:
        """Create an instance of InsertKeyBatch from a dict"""
        if obj is None:
            return None

        if not isinstance(obj, dict):
            return cls.model_validate(obj)

        _obj = cls.model_validate({
            "keys": [InsertKey.from_dict(_item) for _item in obj["keys"]] if obj.get("keys") is not None else None,
            "preserve_order": obj.get("preserve_order")
        })
        return _obj


