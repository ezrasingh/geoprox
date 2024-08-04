# QueryRange

Arguments for range query

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**count** | **int** | Maximum number of neighbors that can be returned (default 100) | [optional] 
**lat** | **float** | Latitude | 
**lng** | **float** | Longitude | 
**range** | **int** | Search radius in kilometers | 
**sorted** | **bool** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

## Example

```python
from geoprox_client.models.query_range import QueryRange

# TODO update the JSON string below
json = "{}"
# create an instance of QueryRange from a JSON string
query_range_instance = QueryRange.from_json(json)
# print the JSON string representation of the object
print(QueryRange.to_json())

# convert the object into a dict
query_range_dict = query_range_instance.to_dict()
# create an instance of QueryRange from a dict
query_range_from_dict = QueryRange.from_dict(query_range_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


