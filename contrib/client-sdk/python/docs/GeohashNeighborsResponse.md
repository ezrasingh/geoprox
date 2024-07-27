# GeohashNeighborsResponse

Neighboring geohash regions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**e** | **str** | East | 
**n** | **str** | North | 
**ne** | **str** | North East | 
**nw** | **str** | North West | 
**s** | **str** | South | 
**se** | **str** | South East | 
**sw** | **str** | South West | 
**w** | **str** | West | 

## Example

```python
from geoprox_client.models.geohash_neighbors_response import GeohashNeighborsResponse

# TODO update the JSON string below
json = "{}"
# create an instance of GeohashNeighborsResponse from a JSON string
geohash_neighbors_response_instance = GeohashNeighborsResponse.from_json(json)
# print the JSON string representation of the object
print(GeohashNeighborsResponse.to_json())

# convert the object into a dict
geohash_neighbors_response_dict = geohash_neighbors_response_instance.to_dict()
# create an instance of GeohashNeighborsResponse from a dict
geohash_neighbors_response_from_dict = GeohashNeighborsResponse.from_dict(geohash_neighbors_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


