# GeoshardApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
|------------- | ------------- | -------------|
| [**createIndex**](GeoshardApiApi.md#createIndex) | **POST** /api/v1/shard/{index} | Create geospatial index |
| [**dropIndex**](GeoshardApiApi.md#dropIndex) | **DELETE** /api/v1/shard/{index} | Drop index |
| [**insertKey**](GeoshardApiApi.md#insertKey) | **PUT** /api/v1/shard/{index} | Insert key into index |
| [**queryRange**](GeoshardApiApi.md#queryRange) | **GET** /api/v1/shard/{index} | Search nearby |
| [**removeKey**](GeoshardApiApi.md#removeKey) | **PATCH** /api/v1/shard/{index} | Remove key from index |


<a name="createIndex"></a>
# **createIndex**
> CreateIndexResponse createIndex(index)

Create geospatial index

    Creates an in-memory index within this geoshard

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **index** | **String**|  | [default to null] |

### Return type

[**CreateIndexResponse**](../Models/CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="dropIndex"></a>
# **dropIndex**
> DropIndexResponse dropIndex(index)

Drop index

    Deletes geospatial index, all keys will be lost

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **index** | **String**|  | [default to null] |

### Return type

[**DropIndexResponse**](../Models/DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="insertKey"></a>
# **insertKey**
> InsertKeyResponse insertKey(index, InsertKey)

Insert key into index

    Inserts key into geospatial index

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **index** | **String**|  | [default to null] |
| **InsertKey** | [**InsertKey**](../Models/InsertKey.md)|  | |

### Return type

[**InsertKeyResponse**](../Models/InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="queryRange"></a>
# **queryRange**
> QueryRangeResponse queryRange(lat, lng, range, index, count, sorted)

Search nearby

    Search geospatial index for all keys within some distance

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **lat** | **Double**| Latitude | [default to null] |
| **lng** | **Double**| Longitude | [default to null] |
| **range** | **Integer**| Search radius in kilometers | [default to null] |
| **index** | **String**|  | [default to null] |
| **count** | **Integer**| Maximum number of neighbors that can be returned (default 100) | [optional] [default to null] |
| **sorted** | **Boolean**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] [default to null] |

### Return type

[**QueryRangeResponse**](../Models/QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="removeKey"></a>
# **removeKey**
> InsertKeyResponse removeKey(index, RemoveKey)

Remove key from index

    Removed key from geospatial index

### Parameters

|Name | Type | Description  | Notes |
|------------- | ------------- | ------------- | -------------|
| **index** | **String**|  | [default to null] |
| **RemoveKey** | [**RemoveKey**](../Models/RemoveKey.md)|  | |

### Return type

[**InsertKeyResponse**](../Models/InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

