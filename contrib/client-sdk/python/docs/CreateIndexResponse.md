# CreateIndexResponse

Returns index creation status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created** | **bool** | If true index was created | 
**existed** | **bool** | If true index alredy exist | 

## Example

```python
from geoprox_client.models.create_index_response import CreateIndexResponse

# TODO update the JSON string below
json = "{}"
# create an instance of CreateIndexResponse from a JSON string
create_index_response_instance = CreateIndexResponse.from_json(json)
# print the JSON string representation of the object
print(CreateIndexResponse.to_json())

# convert the object into a dict
create_index_response_dict = create_index_response_instance.to_dict()
# create an instance of CreateIndexResponse from a dict
create_index_response_from_dict = CreateIndexResponse.from_dict(create_index_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


