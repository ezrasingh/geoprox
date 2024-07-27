# \GeoshardApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index} | Create geospatial index
[**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index} | Drop index
[**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index} | Insert key into index
[**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index} | Search nearby
[**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index} | Remove key from index



## create_index

> models::CreateIndexResponse create_index(index)
Create geospatial index

Creates an in-memory index within this geoshard

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** |  | [required] |

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
Drop index

Deletes geospatial index, all keys will be lost

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** |  | [required] |

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
**index** | **String** |  | [required] |
**insert_key** | [**InsertKey**](InsertKey.md) |  | [required] |

### Return type

[**models::InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_range

> models::QueryRangeResponse query_range(lat, lng, range, index)
Search nearby

Search geospatial index for all keys within some distance

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f64** | latitude | [required] |
**lng** | **f64** | longitude | [required] |
**range** | **i32** | search radius in kilometers | [required] |
**index** | **String** |  | [required] |

### Return type

[**models::QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_key

> models::InsertKeyResponse remove_key(index, remove_key)
Remove key from index

Removed key from geospatial index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**index** | **String** |  | [required] |
**remove_key** | [**RemoveKey**](RemoveKey.md) |  | [required] |

### Return type

[**models::InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

