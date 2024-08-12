# OpenAPI\Client\GeoshardApiApi

All URIs are relative to http://localhost, except if the operation defines another base path.

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createIndex()**](GeoshardApiApi.md#createIndex) | **POST** /api/v1/shard/{index} | Create geospatial index |
| [**dropIndex()**](GeoshardApiApi.md#dropIndex) | **DELETE** /api/v1/shard/{index} | Deletes geospatial index |
| [**insertKey()**](GeoshardApiApi.md#insertKey) | **PUT** /api/v1/shard/{index} | Insert key into index |
| [**insertKeyBatch()**](GeoshardApiApi.md#insertKeyBatch) | **PUT** /api/v1/shard/{index}/batch | Insert multiple keys into index |
| [**queryRange()**](GeoshardApiApi.md#queryRange) | **GET** /api/v1/shard/{index} | Search index for objects nearby |
| [**queryRangeMany()**](GeoshardApiApi.md#queryRangeMany) | **GET** /api/v1/shard | Search multiple indices for objects nearby |
| [**removeKey()**](GeoshardApiApi.md#removeKey) | **PATCH** /api/v1/shard/{index} | Remove key from index |
| [**removeKeyBatch()**](GeoshardApiApi.md#removeKeyBatch) | **PATCH** /api/v1/shard/{index}/batch | Remove multiple keys from index |


## `createIndex()`

```php
createIndex($index): \OpenAPI\Client\Model\CreateIndexResponse
```

Create geospatial index

Creates an in-memory index within this geoshard

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name

try {
    $result = $apiInstance->createIndex($index);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->createIndex: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |

### Return type

[**\OpenAPI\Client\Model\CreateIndexResponse**](../Model/CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `dropIndex()`

```php
dropIndex($index): \OpenAPI\Client\Model\DropIndexResponse
```

Deletes geospatial index

Drop index. All keys will be lost

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name

try {
    $result = $apiInstance->dropIndex($index);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->dropIndex: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |

### Return type

[**\OpenAPI\Client\Model\DropIndexResponse**](../Model/DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `insertKey()`

```php
insertKey($index, $insert_key): \OpenAPI\Client\Model\InsertKeyResponse
```

Insert key into index

Inserts key into geospatial index

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name
$insert_key = new \OpenAPI\Client\Model\InsertKey(); // \OpenAPI\Client\Model\InsertKey | 

try {
    $result = $apiInstance->insertKey($index, $insert_key);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->insertKey: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |
| **insert_key** | [**\OpenAPI\Client\Model\InsertKey**](../Model/InsertKey.md)|  | |

### Return type

[**\OpenAPI\Client\Model\InsertKeyResponse**](../Model/InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `insertKeyBatch()`

```php
insertKeyBatch($index, $insert_key_batch): \OpenAPI\Client\Model\InsertKeyBatchResponse
```

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name
$insert_key_batch = new \OpenAPI\Client\Model\InsertKeyBatch(); // \OpenAPI\Client\Model\InsertKeyBatch | 

try {
    $result = $apiInstance->insertKeyBatch($index, $insert_key_batch);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->insertKeyBatch: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |
| **insert_key_batch** | [**\OpenAPI\Client\Model\InsertKeyBatch**](../Model/InsertKeyBatch.md)|  | |

### Return type

[**\OpenAPI\Client\Model\InsertKeyBatchResponse**](../Model/InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `queryRange()`

```php
queryRange($index, $lat, $lng, $range, $count, $sorted): \OpenAPI\Client\Model\QueryRangeResponse
```

Search index for objects nearby

Search geospatial index for all keys within some distance

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name
$lat = 3.4; // float | Latitude
$lng = 3.4; // float | Longitude
$range = 56; // int | Search radius in kilometers
$count = 56; // int | Maximum number of neighbors that can be returned (default 100)
$sorted = True; // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false)

try {
    $result = $apiInstance->queryRange($index, $lat, $lng, $range, $count, $sorted);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->queryRange: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |
| **lat** | **float**| Latitude | |
| **lng** | **float**| Longitude | |
| **range** | **int**| Search radius in kilometers | |
| **count** | **int**| Maximum number of neighbors that can be returned (default 100) | [optional] |
| **sorted** | **bool**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**\OpenAPI\Client\Model\QueryRangeResponse**](../Model/QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `queryRangeMany()`

```php
queryRangeMany($indices, $lat, $lng, $range, $count, $sorted): \OpenAPI\Client\Model\QueryRangeManyResponse
```

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$indices = array('indices_example'); // string[] | List of indices to search
$lat = 3.4; // float | Latitude
$lng = 3.4; // float | Longitude
$range = 56; // int | Search radius in kilometers
$count = 56; // int | Maximum number of neighbors that can be returned (default 100)
$sorted = True; // bool | If enabled neighbors will be sorted by distance, nearest to furthest (default false)

try {
    $result = $apiInstance->queryRangeMany($indices, $lat, $lng, $range, $count, $sorted);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->queryRangeMany: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **indices** | [**string[]**](../Model/string.md)| List of indices to search | |
| **lat** | **float**| Latitude | |
| **lng** | **float**| Longitude | |
| **range** | **int**| Search radius in kilometers | |
| **count** | **int**| Maximum number of neighbors that can be returned (default 100) | [optional] |
| **sorted** | **bool**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**\OpenAPI\Client\Model\QueryRangeManyResponse**](../Model/QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `removeKey()`

```php
removeKey($index, $remove_key): \OpenAPI\Client\Model\RemoveKeyResponse
```

Remove key from index

Removes key from geospatial index

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name
$remove_key = new \OpenAPI\Client\Model\RemoveKey(); // \OpenAPI\Client\Model\RemoveKey | 

try {
    $result = $apiInstance->removeKey($index, $remove_key);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->removeKey: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |
| **remove_key** | [**\OpenAPI\Client\Model\RemoveKey**](../Model/RemoveKey.md)|  | |

### Return type

[**\OpenAPI\Client\Model\RemoveKeyResponse**](../Model/RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)

## `removeKeyBatch()`

```php
removeKeyBatch($index, $remove_key_batch): \OpenAPI\Client\Model\RemoveKeyBatchResponse
```

Remove multiple keys from index

Removes multiple keys from geospatial index

### Example

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new OpenAPI\Client\Api\GeoshardApiApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);
$index = 'index_example'; // string | Geospatial index name
$remove_key_batch = new \OpenAPI\Client\Model\RemoveKeyBatch(); // \OpenAPI\Client\Model\RemoveKeyBatch | 

try {
    $result = $apiInstance->removeKeyBatch($index, $remove_key_batch);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling GeoshardApiApi->removeKeyBatch: ', $e->getMessage(), PHP_EOL;
}
```

### Parameters

| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **string**| Geospatial index name | |
| **remove_key_batch** | [**\OpenAPI\Client\Model\RemoveKeyBatch**](../Model/RemoveKeyBatch.md)|  | |

### Return type

[**\OpenAPI\Client\Model\RemoveKeyBatchResponse**](../Model/RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: `application/json`
- **Accept**: `application/json`

[[Back to top]](#) [[Back to API list]](../../README.md#endpoints)
[[Back to Model list]](../../README.md#models)
[[Back to README]](../../README.md)
