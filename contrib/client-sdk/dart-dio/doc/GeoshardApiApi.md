# geoprox_client_dio.api.GeoshardApiApi

## Load the API package
```dart
import 'package:geoprox_client_dio/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**createIndex**](GeoshardApiApi.md#createindex) | **POST** /api/v1/shard/{index}/ | Create geospatial index
[**dropIndex**](GeoshardApiApi.md#dropindex) | **DELETE** /api/v1/shard/{index}/ | Deletes geospatial index
[**insertKey**](GeoshardApiApi.md#insertkey) | **PUT** /api/v1/shard/{index}/ | Insert key into index
[**insertKeyBatch**](GeoshardApiApi.md#insertkeybatch) | **PUT** /api/v1/shard/{index}/batch/ | Insert multiple keys into index
[**queryRange**](GeoshardApiApi.md#queryrange) | **GET** /api/v1/shard/{index}/ | Search index for objects nearby
[**queryRangeMany**](GeoshardApiApi.md#queryrangemany) | **GET** /api/v1/shard/ | Search multiple indices for objects nearby
[**removeKey**](GeoshardApiApi.md#removekey) | **PATCH** /api/v1/shard/{index}/ | Remove key from index
[**removeKeyBatch**](GeoshardApiApi.md#removekeybatch) | **PATCH** /api/v1/shard/{index}/batch/ | Remove multiple keys from index


# **createIndex**
> CreateIndexResponse createIndex(index)

Create geospatial index

Creates an in-memory index within this geoshard

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name

try {
    final response = api.createIndex(index);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->createIndex: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **dropIndex**
> DropIndexResponse dropIndex(index)

Deletes geospatial index

Drop index. All keys will be lost

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name

try {
    final response = api.dropIndex(index);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->dropIndex: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 

### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insertKey**
> InsertKeyResponse insertKey(index, insertKey)

Insert key into index

Inserts key into geospatial index

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name
final InsertKey insertKey = ; // InsertKey | 

try {
    final response = api.insertKey(index, insertKey);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->insertKey: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 
 **insertKey** | [**InsertKey**](InsertKey.md)|  | 

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **insertKeyBatch**
> InsertKeyBatchResponse insertKeyBatch(index, insertKeyBatch)

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name
final InsertKeyBatch insertKeyBatch = ; // InsertKeyBatch | 

try {
    final response = api.insertKeyBatch(index, insertKeyBatch);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->insertKeyBatch: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 
 **insertKeyBatch** | [**InsertKeyBatch**](InsertKeyBatch.md)|  | 

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **queryRange**
> QueryRangeResponse queryRange(index, lat, lng, range, count, sorted)

Search index for objects nearby

Search geospatial index for all keys within some distance

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name
final double lat = 1.2; // double | Latitude
final double lng = 1.2; // double | Longitude
final int range = 56; // int | Search radius in kilometers
final int count = 56; // int | Maximum number of neighbors that can be returned (default 100)
final bool sorted = true; // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false)

try {
    final response = api.queryRange(index, lat, lng, range, count, sorted);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->queryRange: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 
 **lat** | **double**| Latitude | 
 **lng** | **double**| Longitude | 
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **queryRangeMany**
> QueryRangeManyResponse queryRangeMany(indices, lat, lng, range, count, sorted)

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final BuiltList<String> indices = ; // BuiltList<String> | List of indices to search
final double lat = 1.2; // double | Latitude
final double lng = 1.2; // double | Longitude
final int range = 56; // int | Search radius in kilometers
final int count = 56; // int | Maximum number of neighbors that can be returned (default 100)
final bool sorted = true; // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false)

try {
    final response = api.queryRangeMany(indices, lat, lng, range, count, sorted);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->queryRangeMany: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **indices** | [**BuiltList&lt;String&gt;**](String.md)| List of indices to search | 
 **lat** | **double**| Latitude | 
 **lng** | **double**| Longitude | 
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

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeKey**
> RemoveKeyResponse removeKey(index, removeKey)

Remove key from index

Removes key from geospatial index

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name
final RemoveKey removeKey = ; // RemoveKey | 

try {
    final response = api.removeKey(index, removeKey);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->removeKey: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 
 **removeKey** | [**RemoveKey**](RemoveKey.md)|  | 

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **removeKeyBatch**
> RemoveKeyBatchResponse removeKeyBatch(index, removeKeyBatch)

Remove multiple keys from index

Removes multiple keys from geospatial index

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeoshardApiApi();
final String index = index_example; // String | Geospatial index name
final RemoveKeyBatch removeKeyBatch = ; // RemoveKeyBatch | 

try {
    final response = api.removeKeyBatch(index, removeKeyBatch);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeoshardApiApi->removeKeyBatch: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **index** | **String**| Geospatial index name | 
 **removeKeyBatch** | [**RemoveKeyBatch**](RemoveKeyBatch.md)|  | 

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

