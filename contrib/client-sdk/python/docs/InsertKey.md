# InsertKey

Arguments for inserting a key

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **str** | Object key | 
**lat** | **float** | Latitude | 
**lng** | **float** | Longitude | 

## Example

```python
from geoprox_client.models.insert_key import InsertKey

# TODO update the JSON string below
json = "{}"
# create an instance of InsertKey from a JSON string
insert_key_instance = InsertKey.from_json(json)
# print the JSON string representation of the object
print(InsertKey.to_json())

# convert the object into a dict
insert_key_dict = insert_key_instance.to_dict()
# create an instance of InsertKey from a dict
insert_key_from_dict = InsertKey.from_dict(insert_key_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


