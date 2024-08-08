# InsertKeyBatchResponse

Returns results and errors of batch key insert

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**errors** | **Dict[str, str]** | Contains information about which keys failed to be inserted and the associated error details. | 
**results** | **Dict[str, str]** | Object keys that have been inserted in the index and their geohash. | 

## Example

```python
from geoprox_client.models.insert_key_batch_response import InsertKeyBatchResponse

# TODO update the JSON string below
json = "{}"
# create an instance of InsertKeyBatchResponse from a JSON string
insert_key_batch_response_instance = InsertKeyBatchResponse.from_json(json)
# print the JSON string representation of the object
print(InsertKeyBatchResponse.to_json())

# convert the object into a dict
insert_key_batch_response_dict = insert_key_batch_response_instance.to_dict()
# create an instance of InsertKeyBatchResponse from a dict
insert_key_batch_response_from_dict = InsertKeyBatchResponse.from_dict(insert_key_batch_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


