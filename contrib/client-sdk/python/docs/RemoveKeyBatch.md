# RemoveKeyBatch

Arguments for removing multiple keys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**keys** | **List[str]** | Object key | 

## Example

```python
from geoprox_client.models.remove_key_batch import RemoveKeyBatch

# TODO update the JSON string below
json = "{}"
# create an instance of RemoveKeyBatch from a JSON string
remove_key_batch_instance = RemoveKeyBatch.from_json(json)
# print the JSON string representation of the object
print(RemoveKeyBatch.to_json())

# convert the object into a dict
remove_key_batch_dict = remove_key_batch_instance.to_dict()
# create an instance of RemoveKeyBatch from a dict
remove_key_batch_from_dict = RemoveKeyBatch.from_dict(remove_key_batch_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


