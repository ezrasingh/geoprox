# \GeohashApiApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**decode_geohash**](GeohashApiApi.md#decode_geohash) | **GET** /api/v1/geohash/{ghash} | Decode geohash into coordinates.
[**encode_latlng**](GeohashApiApi.md#encode_latlng) | **GET** /api/v1/geohash | Encode coordinates into geohash
[**get_neighbors**](GeohashApiApi.md#get_neighbors) | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions



## decode_geohash

> models::DecodeGeohashResponse decode_geohash(ghash)
Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ghash** | **String** | Geohash encoded region | [required] |

### Return type

[**models::DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## encode_latlng

> models::EncodeLatLngResponse encode_latlng(lat, lng, depth)
Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f64** | Latitude | [required] |
**lng** | **f64** | Longitude | [required] |
**depth** | **i32** | Determines geohash length | [required] |

### Return type

[**models::EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_neighbors

> models::GeohashNeighborsResponse get_neighbors(ghash)
Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ghash** | **String** | Geohash encoded region | [required] |

### Return type

[**models::GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

