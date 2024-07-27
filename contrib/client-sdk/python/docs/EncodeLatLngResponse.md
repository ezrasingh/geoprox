# EncodeLatLngResponse

Returns geohash encoded latitude/longitude

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**geohash** | **str** |  | 

## Example

```python
from geoprox_client.models.encode_lat_lng_response import EncodeLatLngResponse

# TODO update the JSON string below
json = "{}"
# create an instance of EncodeLatLngResponse from a JSON string
encode_lat_lng_response_instance = EncodeLatLngResponse.from_json(json)
# print the JSON string representation of the object
print(EncodeLatLngResponse.to_json())

# convert the object into a dict
encode_lat_lng_response_dict = encode_lat_lng_response_instance.to_dict()
# create an instance of EncodeLatLngResponse from a dict
encode_lat_lng_response_from_dict = EncodeLatLngResponse.from_dict(encode_lat_lng_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


