# QueryRangeManyResponse

Returns indices and object keys found with their distance

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | **Dict[str, str]** | Contains information about any errors occured during batch search. | 
**results** | **Dict[str, List[Neighbor]]** | Object keys found within range | 

## Example

```python
from geoprox_client.models.query_range_many_response import QueryRangeManyResponse

# TODO update the JSON string below
json = "{}"
# create an instance of QueryRangeManyResponse from a JSON string
query_range_many_response_instance = QueryRangeManyResponse.from_json(json)
# print the JSON string representation of the object
print(QueryRangeManyResponse.to_json())

# convert the object into a dict
query_range_many_response_dict = query_range_many_response_instance.to_dict()
# create an instance of QueryRangeManyResponse from a dict
query_range_many_response_from_dict = QueryRangeManyResponse.from_dict(query_range_many_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


