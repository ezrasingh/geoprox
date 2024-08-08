# geoprox_client.GeoshardApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index}/ | Create geospatial index
[**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index}/ | Deletes geospatial index
[**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index}/ | Insert key into index
[**insert_key_batch**](GeoshardApiApi.md#insert_key_batch) | **PUT** /api/v1/shard/{index}/batch/ | Insert multiple keys into index
[**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index}/ | Search index for objects nearby
[**query_range_many**](GeoshardApiApi.md#query_range_many) | **GET** /api/v1/shard/ | Search multiple indices for objects nearby
[**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index}/ | Remove key from index
[**remove_key_batch**](GeoshardApiApi.md#remove_key_batch) | **PATCH** /api/v1/shard/{index}/batch/ | Remove multiple keys from index


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
    index = 'index_example' # str | Geospatial index name

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
 **index** | **str**| Geospatial index name | 

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

Deletes geospatial index

Drop index. All keys will be lost

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
    index = 'index_example' # str | Geospatial index name

    try:
        # Deletes geospatial index
        api_response = api_instance.drop_index(index)
        print("The response of GeoshardApiApi->drop_index:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->drop_index: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**| Geospatial index name | 

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
    index = 'index_example' # str | Geospatial index name
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
 **index** | **str**| Geospatial index name | 
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
**201** | Inserted key into the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insert_key_batch**
> InsertKeyBatchResponse insert_key_batch(index, insert_key_batch)

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Example


```python
import geoprox_client
from geoprox_client.models.insert_key_batch import InsertKeyBatch
from geoprox_client.models.insert_key_batch_response import InsertKeyBatchResponse
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
    index = 'index_example' # str | Geospatial index name
    insert_key_batch = geoprox_client.InsertKeyBatch() # InsertKeyBatch | 

    try:
        # Insert multiple keys into index
        api_response = api_instance.insert_key_batch(index, insert_key_batch)
        print("The response of GeoshardApiApi->insert_key_batch:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->insert_key_batch: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**| Geospatial index name | 
 **insert_key_batch** | [**InsertKeyBatch**](InsertKeyBatch.md)|  | 

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**201** | Inserted key batch into the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **query_range**
> QueryRangeResponse query_range(index, lat, lng, range, count=count, sorted=sorted)

Search index for objects nearby

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
    index = 'index_example' # str | Geospatial index name
    lat = 3.4 # float | Latitude
    lng = 3.4 # float | Longitude
    range = 56 # int | Search radius in kilometers
    count = 56 # int | Maximum number of neighbors that can be returned (default 100) (optional)
    sorted = True # bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional)

    try:
        # Search index for objects nearby
        api_response = api_instance.query_range(index, lat, lng, range, count=count, sorted=sorted)
        print("The response of GeoshardApiApi->query_range:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->query_range: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**| Geospatial index name | 
 **lat** | **float**| Latitude | 
 **lng** | **float**| Longitude | 
 **range** | **int**| Search radius in kilometers | 
 **count** | **int**| Maximum number of neighbors that can be returned (default 100) | [optional] 
 **sorted** | **bool**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

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
**200** | Nearby objects found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **query_range_many**
> QueryRangeManyResponse query_range_many(indices, lat, lng, range, count=count, sorted=sorted)

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Example


```python
import geoprox_client
from geoprox_client.models.query_range_many_response import QueryRangeManyResponse
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
    indices = ['indices_example'] # List[str] | List of indices to search
    lat = 3.4 # float | Latitude
    lng = 3.4 # float | Longitude
    range = 56 # int | Search radius in kilometers
    count = 56 # int | Maximum number of neighbors that can be returned (default 100) (optional)
    sorted = True # bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false) (optional)

    try:
        # Search multiple indices for objects nearby
        api_response = api_instance.query_range_many(indices, lat, lng, range, count=count, sorted=sorted)
        print("The response of GeoshardApiApi->query_range_many:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->query_range_many: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **indices** | [**List[str]**](str.md)| List of indices to search | 
 **lat** | **float**| Latitude | 
 **lng** | **float**| Longitude | 
 **range** | **int**| Search radius in kilometers | 
 **count** | **int**| Maximum number of neighbors that can be returned (default 100) | [optional] 
 **sorted** | **bool**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] 

### Return type

[**QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Nearby objects found across indices |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_key**
> RemoveKeyResponse remove_key(index, remove_key)

Remove key from index

Removes key from geospatial index

### Example


```python
import geoprox_client
from geoprox_client.models.remove_key import RemoveKey
from geoprox_client.models.remove_key_response import RemoveKeyResponse
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
    index = 'index_example' # str | Geospatial index name
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
 **index** | **str**| Geospatial index name | 
 **remove_key** | [**RemoveKey**](RemoveKey.md)|  | 

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | Key removed from the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **remove_key_batch**
> RemoveKeyBatchResponse remove_key_batch(index, remove_key_batch)

Remove multiple keys from index

Removes multiple keys from geospatial index

### Example


```python
import geoprox_client
from geoprox_client.models.remove_key_batch import RemoveKeyBatch
from geoprox_client.models.remove_key_batch_response import RemoveKeyBatchResponse
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
    index = 'index_example' # str | Geospatial index name
    remove_key_batch = geoprox_client.RemoveKeyBatch() # RemoveKeyBatch | 

    try:
        # Remove multiple keys from index
        api_response = api_instance.remove_key_batch(index, remove_key_batch)
        print("The response of GeoshardApiApi->remove_key_batch:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling GeoshardApiApi->remove_key_batch: %s\n" % e)
```



### Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **str**| Geospatial index name | 
 **remove_key_batch** | [**RemoveKeyBatch**](RemoveKeyBatch.md)|  | 

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

### HTTP response details

| Status code | Description | Response headers |
|-------------|-------------|------------------|
**200** | All keys were removed from the index |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

