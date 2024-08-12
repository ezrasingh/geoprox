# \GeoshardApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index} | Create geospatial index
[**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index} | Deletes geospatial index
[**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index} | Insert key into index
[**insert_key_batch**](GeoshardApiApi.md#insert_key_batch) | **PUT** /api/v1/shard/{index}/batch | Insert multiple keys into index
[**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index} | Search index for objects nearby
[**query_range_many**](GeoshardApiApi.md#query_range_many) | **GET** /api/v1/shard | Search multiple indices for objects nearby
[**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index} | Remove key from index
[**remove_key_batch**](GeoshardApiApi.md#remove_key_batch) | **PATCH** /api/v1/shard/{index}/batch | Remove multiple keys from index



## create_index

> models::CreateIndexResponse create_index(index)
Create geospatial index

Creates an in-memory index within this geoshard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |

### Return type

[**models::CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## drop_index

> models::DropIndexResponse drop_index(index)
Deletes geospatial index

Drop index. All keys will be lost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |

### Return type

[**models::DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert_key

> models::InsertKeyResponse insert_key(index, insert_key)
Insert key into index

Inserts key into geospatial index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |
**insert_key** | [**InsertKey**](InsertKey.md) |  | [required] |

### Return type

[**models::InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## insert_key_batch

> models::InsertKeyBatchResponse insert_key_batch(index, insert_key_batch)
Insert multiple keys into index

Inserts multiple keys into geospatial index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |
**insert_key_batch** | [**InsertKeyBatch**](InsertKeyBatch.md) |  | [required] |

### Return type

[**models::InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_range

> models::QueryRangeResponse query_range(index, lat, lng, range, count, sorted)
Search index for objects nearby

Search geospatial index for all keys within some distance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |
**lat** | **f64** | Latitude | [required] |
**lng** | **f64** | Longitude | [required] |
**range** | **i32** | Search radius in kilometers | [required] |
**count** | Option<**i32**> | Maximum number of neighbors that can be returned (default 100) |  |
**sorted** | Option<**bool**> | If enabled neighbors will be sorted by distance, nearest to furthest (default false) |  |

### Return type

[**models::QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_range_many

> models::QueryRangeManyResponse query_range_many(indices, lat, lng, range, count, sorted)
Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**indices** | [**Vec<String>**](String.md) | List of indices to search | [required] |
**lat** | **f64** | Latitude | [required] |
**lng** | **f64** | Longitude | [required] |
**range** | **i32** | Search radius in kilometers | [required] |
**count** | Option<**i32**> | Maximum number of neighbors that can be returned (default 100) |  |
**sorted** | Option<**bool**> | If enabled neighbors will be sorted by distance, nearest to furthest (default false) |  |

### Return type

[**models::QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_key

> models::RemoveKeyResponse remove_key(index, remove_key)
Remove key from index

Removes key from geospatial index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |
**remove_key** | [**RemoveKey**](RemoveKey.md) |  | [required] |

### Return type

[**models::RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_key_batch

> models::RemoveKeyBatchResponse remove_key_batch(index, remove_key_batch)
Remove multiple keys from index

Removes multiple keys from geospatial index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** | Geospatial index name | [required] |
**remove_key_batch** | [**RemoveKeyBatch**](RemoveKeyBatch.md) |  | [required] |

### Return type

[**models::RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

