# InsertKeyBatch

Arguments for inserting multiple keys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**keys** | [**List[InsertKey]**](InsertKey.md) | Object key | 
**preserve_order** | **bool** |  | 

## Example

```python
from geoprox_client.models.insert_key_batch import InsertKeyBatch

# TODO update the JSON string below
json = "{}"
# create an instance of InsertKeyBatch from a JSON string
insert_key_batch_instance = InsertKeyBatch.from_json(json)
# print the JSON string representation of the object
print(InsertKeyBatch.to_json())

# convert the object into a dict
insert_key_batch_dict = insert_key_batch_instance.to_dict()
# create an instance of InsertKeyBatch from a dict
insert_key_batch_from_dict = InsertKeyBatch.from_dict(insert_key_batch_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


