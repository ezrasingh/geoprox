# geoprox_client.GeohashApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_geohash**](GeohashApiApi.md#decode_geohash) | **GET** /api/v1/geohash/{ghash}/ | Decode geohash into coordinates.
[**encode_latlng**](GeohashApiApi.md#encode_latlng) | **GET** /api/v1/geohash/ | Encode coordinates into geohash
[**get_neighbors**](GeohashApiApi.md#get_neighbors) | **GET** /api/v1/geohash/{ghash}/neighbors/ | Neighboring regions


# **decode_geohash**
> DecodeGeohashResponse decode_geohash(ghash)

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Example


```python
import geoprox_client
from geoprox_client.models.decode_geohash_response import DecodeGeohashResponse
from geoprox_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = geoprox_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with geoprox_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = geoprox_client.GeohashApiApi(api_client)
    ghash = 'ghash_example' # str | Geohash encoded region

    try:
        # Decode geohash into coordinates.
        api_response = api_instance.decode_geohash(ghash)
        print("The response of GeohashApiApi->decode_geohash:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeohashApiApi->decode_geohash: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ghash** | **str**| Geohash encoded region | 

### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Object with latitude/longitude pair and precision errors |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encode_latlng**
> EncodeLatLngResponse encode_latlng(lat, lng, depth)

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Example


```python
import geoprox_client
from geoprox_client.models.encode_lat_lng_response import EncodeLatLngResponse
from geoprox_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = geoprox_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with geoprox_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = geoprox_client.GeohashApiApi(api_client)
    lat = 3.4 # float | Latitude
    lng = 3.4 # float | Longitude
    depth = 56 # int | Determines geohash length

    try:
        # Encode coordinates into geohash
        api_response = api_instance.encode_latlng(lat, lng, depth)
        print("The response of GeohashApiApi->encode_latlng:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeohashApiApi->encode_latlng: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lat** | **float**| Latitude | 
 **lng** | **float**| Longitude | 
 **depth** | **int**| Determines geohash length | 

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Object with geohash encoded latitude/longitude |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **get_neighbors**
> GeohashNeighborsResponse get_neighbors(ghash)

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Example


```python
import geoprox_client
from geoprox_client.models.geohash_neighbors_response import GeohashNeighborsResponse
from geoprox_client.rest import ApiException
from pprint import pprint

# Defining the host is optional and defaults to http://localhost
# See configuration.py for a list of all supported configuration parameters.
configuration = geoprox_client.Configuration(
    host = "http://localhost"
)


# Enter a context with an instance of the API client
with geoprox_client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = geoprox_client.GeohashApiApi(api_client)
    ghash = 'ghash_example' # str | Geohash encoded region

    try:
        # Neighboring regions
        api_response = api_instance.get_neighbors(ghash)
        print("The response of GeohashApiApi->get_neighbors:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeohashApiApi->get_neighbors: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ghash** | **str**| Geohash encoded region | 

### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Object with geohash neighbors |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

