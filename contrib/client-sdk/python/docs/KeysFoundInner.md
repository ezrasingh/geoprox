# KeysFoundInner


## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**distance** | **float** | Distance in kilometers | [optional] 
**key** | **str** |  | [optional] 

## Example

```python
from geoprox_client.models.keys_found_inner import KeysFoundInner

# TODO update the JSON string below
json = "{}"
# create an instance of KeysFoundInner from a JSON string
keys_found_inner_instance = KeysFoundInner.from_json(json)
# print the JSON string representation of the object
print(KeysFoundInner.to_json())

# convert the object into a dict
keys_found_inner_dict = keys_found_inner_instance.to_dict()
# create an instance of KeysFoundInner from a dict
keys_found_inner_from_dict = KeysFoundInner.from_dict(keys_found_inner_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


