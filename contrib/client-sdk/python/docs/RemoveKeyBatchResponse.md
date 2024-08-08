# RemoveKeyBatchResponse

Returns batch key deletion status

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**deleted** | **bool** | If true all keys were removed | 

## Example

```python
from geoprox_client.models.remove_key_batch_response import RemoveKeyBatchResponse

# TODO update the JSON string below
json = "{}"
# create an instance of RemoveKeyBatchResponse from a JSON string
remove_key_batch_response_instance = RemoveKeyBatchResponse.from_json(json)
# print the JSON string representation of the object
print(RemoveKeyBatchResponse.to_json())

# convert the object into a dict
remove_key_batch_response_dict = remove_key_batch_response_instance.to_dict()
# create an instance of RemoveKeyBatchResponse from a dict
remove_key_batch_response_from_dict = RemoveKeyBatchResponse.from_dict(remove_key_batch_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


