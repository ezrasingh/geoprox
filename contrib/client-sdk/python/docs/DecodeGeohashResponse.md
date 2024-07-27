# DecodeGeohashResponse

Returns geohash decoded as latitude/longitude with precision errors

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**lat** | **float** | Latitude | 
**lat_error** | **float** | Latitude error | 
**lng** | **float** | Longitude | 
**lng_error** | **float** | Longitude error | 

## Example

```python
from geoprox_client.models.decode_geohash_response import DecodeGeohashResponse

# TODO update the JSON string below
json = "{}"
# create an instance of DecodeGeohashResponse from a JSON string
decode_geohash_response_instance = DecodeGeohashResponse.from_json(json)
# print the JSON string representation of the object
print(DecodeGeohashResponse.to_json())

# convert the object into a dict
decode_geohash_response_dict = decode_geohash_response_instance.to_dict()
# create an instance of DecodeGeohashResponse from a dict
decode_geohash_response_from_dict = DecodeGeohashResponse.from_dict(decode_geohash_response_dict)
```
[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


