# OpenAPI\Client\GeohashApiApi

All URIs are relative to http://localhost, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**decodeGeohash()**](GeohashApiApi.md#decodeGeohash) | **GET** /api/v1/geohash/{ghash} | Decode geohash into coordinates. |
| [**encodeLatlng()**](GeohashApiApi.md#encodeLatlng) | **GET** /api/v1/geohash | Encode coordinates into geohash |
| [**getNeighbors()**](GeohashApiApi.md#getNeighbors) | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions |


## `decodeGeohash()`

```php
decodeGeohash($ghash): \OpenAPI\Client\Model\DecodeGeohashResponse
```

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeohashApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$ghash = 'ghash_example'; // string | Geohash encoded region

try {
    $result = $apiInstance->decodeGeohash($ghash);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeohashApiApi->decodeGeohash: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **ghash** | **string**| Geohash encoded region | |

### Return type

[**\OpenAPI\Client\Model\DecodeGeohashResponse**](../Model/DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `encodeLatlng()`

```php
encodeLatlng($lat, $lng, $depth): \OpenAPI\Client\Model\EncodeLatLngResponse
```

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeohashApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$lat = 3.4; // float | Latitude
$lng = 3.4; // float | Longitude
$depth = 56; // int | Determines geohash length

try {
    $result = $apiInstance->encodeLatlng($lat, $lng, $depth);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeohashApiApi->encodeLatlng: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **lat** | **float**| Latitude | |
| **lng** | **float**| Longitude | |
| **depth** | **int**| Determines geohash length | |

### Return type

[**\OpenAPI\Client\Model\EncodeLatLngResponse**](../Model/EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `getNeighbors()`

```php
getNeighbors($ghash): \OpenAPI\Client\Model\GeohashNeighborsResponse
```

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeohashApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$ghash = 'ghash_example'; // string | Geohash encoded region

try {
    $result = $apiInstance->getNeighbors($ghash);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeohashApiApi->getNeighbors: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **ghash** | **string**| Geohash encoded region | |

### Return type

[**\OpenAPI\Client\Model\GeohashNeighborsResponse**](../Model/GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)
