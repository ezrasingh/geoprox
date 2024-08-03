# Neighbor

Nearby resource key

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**distance** | **float** | Distance in kilometers | 
**key** | **str** | Resource key | 

## Example

```python
from geoprox_client.models.neighbor import Neighbor

# TODO update the JSON string below
json = "{}"
# create an instance of Neighbor from a JSON string
neighbor_instance = Neighbor.from_json(json)
# print the JSON string representation of the object
print(Neighbor.to_json())

# convert the object into a dict
neighbor_dict = neighbor_instance.to_dict()
# create an instance of Neighbor from a dict
neighbor_from_dict = Neighbor.from_dict(neighbor_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


