# RemoveKey

Arguments for removing a key

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Resource key | 

## Example

```python
from geoprox_client.models.remove_key import RemoveKey

# TODO update the JSON string below
json = "{}"
# create an instance of RemoveKey from a JSON string
remove_key_instance = RemoveKey.from_json(json)
# print the JSON string representation of the object
print(RemoveKey.to_json())

# convert the object into a dict
remove_key_dict = remove_key_instance.to_dict()
# create an instance of RemoveKey from a dict
remove_key_from_dict = RemoveKey.from_dict(remove_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


