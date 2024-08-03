# InsertKeyResponse

Returns key and geohash

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**geohash** | **str** | Geohash encoded latitude/longitude | 
**key** | **str** | Resource key | 

## Example

```python
from geoprox_client.models.insert_key_response import InsertKeyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of InsertKeyResponse from a JSON string
insert_key_response_instance = InsertKeyResponse.from_json(json)
# print the JSON string representation of the object
print(InsertKeyResponse.to_json())

# convert the object into a dict
insert_key_response_dict = insert_key_response_instance.to_dict()
# create an instance of InsertKeyResponse from a dict
insert_key_response_from_dict = InsertKeyResponse.from_dict(insert_key_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


