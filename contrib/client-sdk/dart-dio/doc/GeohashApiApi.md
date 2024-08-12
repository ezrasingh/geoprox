# geoprox_client_dio.api.GeohashApiApi

## Load the API package
```dart
import 'package:geoprox_client_dio/api.dart';
```

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decodeGeohash**](GeohashApiApi.md#decodegeohash) | **GET** /api/v1/geohash/{ghash} | Decode geohash into coordinates.
[**encodeLatlng**](GeohashApiApi.md#encodelatlng) | **GET** /api/v1/geohash | Encode coordinates into geohash
[**getNeighbors**](GeohashApiApi.md#getneighbors) | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions


# **decodeGeohash**
> DecodeGeohashResponse decodeGeohash(ghash)

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeohashApiApi();
final String ghash = ghash_example; // String | Geohash encoded region

try {
    final response = api.decodeGeohash(ghash);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeohashApiApi->decodeGeohash: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ghash** | **String**| Geohash encoded region | 

### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **encodeLatlng**
> EncodeLatLngResponse encodeLatlng(lat, lng, depth)

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeohashApiApi();
final double lat = 1.2; // double | Latitude
final double lng = 1.2; // double | Longitude
final int depth = 56; // int | Determines geohash length

try {
    final response = api.encodeLatlng(lat, lng, depth);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeohashApiApi->encodeLatlng: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **lat** | **double**| Latitude | 
 **lng** | **double**| Longitude | 
 **depth** | **int**| Determines geohash length | 

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getNeighbors**
> GeohashNeighborsResponse getNeighbors(ghash)

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Example
```dart
import 'package:geoprox_client_dio/api.dart';

final api = GeoproxClientDio().getGeohashApiApi();
final String ghash = ghash_example; // String | Geohash encoded region

try {
    final response = api.getNeighbors(ghash);
    print(response);
} catch on DioException (e) {
    print('Exception when calling GeohashApiApi->getNeighbors: $e\n');
}
```

### Parameters

Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ghash** | **String**| Geohash encoded region | 

### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

