# EncodeLatLng

Arguments for encoding latitude/longitude as geohash

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**depth** | **int** | Determines geohash length | 
**lat** | **float** | Latitude | 
**lng** | **float** | Longitude | 

## Example

```python
from geoprox_client.models.encode_lat_lng import EncodeLatLng

# TODO update the JSON string below
json = "{}"
# create an instance of EncodeLatLng from a JSON string
encode_lat_lng_instance = EncodeLatLng.from_json(json)
# print the JSON string representation of the object
print(EncodeLatLng.to_json())

# convert the object into a dict
encode_lat_lng_dict = encode_lat_lng_instance.to_dict()
# create an instance of EncodeLatLng from a dict
encode_lat_lng_from_dict = EncodeLatLng.from_dict(encode_lat_lng_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


