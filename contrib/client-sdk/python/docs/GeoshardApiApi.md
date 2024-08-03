# geoprox_client.GeoshardApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index} | Create geospatial index
[**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index} | Drop index
[**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index} | Insert key into index
[**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index} | Search nearby
[**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index} | Remove key from index


# **create_index**
> CreateIndexResponse create_index(index)

Create geospatial index

Creates an in-memory index within this geoshard

### Example


```python
import geoprox_client
from geoprox_client.models.create_index_response import CreateIndexResponse
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
    api_instance = geoprox_client.GeoshardApiApi(api_client)
    index = 'index_example' # str | 

    try:
        # Create geospatial index
        api_response = api_instance.create_index(index)
        print("The response of GeoshardApiApi->create_index:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->create_index: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**|  | 

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Created an index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **drop_index**
> DropIndexResponse drop_index(index)

Drop index

Deletes geospatial index, all keys will be lost

### Example


```python
import geoprox_client
from geoprox_client.models.drop_index_response import DropIndexResponse
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
    api_instance = geoprox_client.GeoshardApiApi(api_client)
    index = 'index_example' # str | 

    try:
        # Drop index
        api_response = api_instance.drop_index(index)
        print("The response of GeoshardApiApi->drop_index:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->drop_index: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**|  | 

### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**202** | Index deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insert_key**
> InsertKeyResponse insert_key(index, insert_key)

Insert key into index

Inserts key into geospatial index

### Example


```python
import geoprox_client
from geoprox_client.models.insert_key import InsertKey
from geoprox_client.models.insert_key_response import InsertKeyResponse
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
    api_instance = geoprox_client.GeoshardApiApi(api_client)
    index = 'index_example' # str | 
    insert_key = geoprox_client.InsertKey() # InsertKey | 

    try:
        # Insert key into index
        api_response = api_instance.insert_key(index, insert_key)
        print("The response of GeoshardApiApi->insert_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->insert_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**|  | 
 **insert_key** | [**InsertKey**](InsertKey.md)|  | 

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Inserted key into index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **query_range**
> QueryRangeResponse query_range(lat, lng, range, index)

Search nearby

Search geospatial index for all keys within some distance

### Example


```python
import geoprox_client
from geoprox_client.models.query_range_response import QueryRangeResponse
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
    api_instance = geoprox_client.GeoshardApiApi(api_client)
    lat = 3.4 # float | Latitude
    lng = 3.4 # float | Longitude
    range = 56 # int | Search radius in kilometers
    index = 'index_example' # str | 

    try:
        # Search nearby
        api_response = api_instance.query_range(lat, lng, range, index)
        print("The response of GeoshardApiApi->query_range:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->query_range: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lat** | **float**| Latitude | 
 **lng** | **float**| Longitude | 
 **range** | **int**| Search radius in kilometers | 
 **index** | **str**|  | 

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Index deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_key**
> InsertKeyResponse remove_key(index, remove_key)

Remove key from index

Removed key from geospatial index

### Example


```python
import geoprox_client
from geoprox_client.models.insert_key_response import InsertKeyResponse
from geoprox_client.models.remove_key import RemoveKey
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
    api_instance = geoprox_client.GeoshardApiApi(api_client)
    index = 'index_example' # str | 
    remove_key = geoprox_client.RemoveKey() # RemoveKey | 

    try:
        # Remove key from index
        api_response = api_instance.remove_key(index, remove_key)
        print("The response of GeoshardApiApi->remove_key:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->remove_key: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**|  | 
 **remove_key** | [**RemoveKey**](RemoveKey.md)|  | 

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Key removed from index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

