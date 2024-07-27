# QueryRangeResponse

Returns resource keys found with their distance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**found** | [**List[KeysFoundInner]**](KeysFoundInner.md) | Resource keys found within range | 

## Example

```python
from geoprox_client.models.query_range_response import QueryRangeResponse

# TODO update the JSON string below
json = "{}"
# create an instance of QueryRangeResponse from a JSON string
query_range_response_instance = QueryRangeResponse.from_json(json)
# print the JSON string representation of the object
print(QueryRangeResponse.to_json())

# convert the object into a dict
query_range_response_dict = query_range_response_instance.to_dict()
# create an instance of QueryRangeResponse from a dict
query_range_response_from_dict = QueryRangeResponse.from_dict(query_range_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


