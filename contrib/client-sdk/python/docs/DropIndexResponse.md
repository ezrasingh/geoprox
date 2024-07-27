# DropIndexResponse

Returns index deletion status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | **bool** | If true index was deleted | 

## Example

```python
from geoprox_client.models.drop_index_response import DropIndexResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DropIndexResponse from a JSON string
drop_index_response_instance = DropIndexResponse.from_json(json)
# print the JSON string representation of the object
print(DropIndexResponse.to_json())

# convert the object into a dict
drop_index_response_dict = drop_index_response_instance.to_dict()
# create an instance of DropIndexResponse from a dict
drop_index_response_from_dict = DropIndexResponse.from_dict(drop_index_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


