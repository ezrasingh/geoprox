# GeoshardApi

All URIs are relative to _http://localhost_

| Method                                              | HTTP request                          | Description                                |
| --------------------------------------------------- | ------------------------------------- | ------------------------------------------ |
| [**createIndex**](GeoshardApi.md#createIndex)       | **POST** /api/v1/shard/{index}        | Create geospatial index                    |
| [**dropIndex**](GeoshardApi.md#dropIndex)           | **DELETE** /api/v1/shard/{index}      | Deletes geospatial index                   |
| [**insertKey**](GeoshardApi.md#insertKey)           | **PUT** /api/v1/shard/{index}         | Insert key into index                      |
| [**insertKeyBatch**](GeoshardApi.md#insertKeyBatch) | **PUT** /api/v1/shard/{index}/batch   | Insert multiple keys into index            |
| [**queryRange**](GeoshardApi.md#queryRange)         | **GET** /api/v1/shard/{index}         | Search index for objects nearby            |
| [**queryRangeMany**](GeoshardApi.md#queryRangeMany) | **GET** /api/v1/shard                 | Search multiple indices for objects nearby |
| [**removeKey**](GeoshardApi.md#removeKey)           | **PATCH** /api/v1/shard/{index}       | Remove key from index                      |
| [**removeKeyBatch**](GeoshardApi.md#removeKeyBatch) | **PATCH** /api/v1/shard/{index}/batch | Remove multiple keys from index            |

<a name="createIndex"></a>

# **createIndex**

> CreateIndexResponse createIndex(index)

Create geospatial index

    Creates an in-memory index within this geoshard

### Parameters

| Name      | Type       | Description           | Notes             |
| --------- | ---------- | --------------------- | ----------------- |
| **index** | **String** | Geospatial index name | [default to null] |

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

Deletes geospatial index

    Drop index. All keys will be lost

### Parameters

| Name      | Type       | Description           | Notes             |
| --------- | ---------- | --------------------- | ----------------- |
| **index** | **String** | Geospatial index name | [default to null] |

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

| Name          | Type                                    | Description           | Notes             |
| ------------- | --------------------------------------- | --------------------- | ----------------- |
| **index**     | **String**                              | Geospatial index name | [default to null] |
| **InsertKey** | [**InsertKey**](../Models/InsertKey.md) |                       |                   |

### Return type

[**InsertKeyResponse**](../Models/InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="insertKeyBatch"></a>

# **insertKeyBatch**

> InsertKeyBatchResponse insertKeyBatch(index, InsertKeyBatch)

Insert multiple keys into index

    Inserts multiple keys into geospatial index

### Parameters

| Name               | Type                                              | Description           | Notes             |
| ------------------ | ------------------------------------------------- | --------------------- | ----------------- |
| **index**          | **String**                                        | Geospatial index name | [default to null] |
| **InsertKeyBatch** | [**InsertKeyBatch**](../Models/InsertKeyBatch.md) |                       |                   |

### Return type

[**InsertKeyBatchResponse**](../Models/InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="queryRange"></a>

# **queryRange**

> QueryRangeResponse queryRange(index, lat, lng, range, count, sorted)

Search index for objects nearby

    Search geospatial index for all keys within some distance

### Parameters

| Name       | Type        | Description                                                                          | Notes                        |
| ---------- | ----------- | ------------------------------------------------------------------------------------ | ---------------------------- |
| **index**  | **String**  | Geospatial index name                                                                | [default to null]            |
| **lat**    | **Double**  | Latitude                                                                             | [default to null]            |
| **lng**    | **Double**  | Longitude                                                                            | [default to null]            |
| **range**  | **Integer** | Search radius in kilometers                                                          | [default to null]            |
| **count**  | **Integer** | Maximum number of neighbors that can be returned (default 100)                       | [optional] [default to null] |
| **sorted** | **Boolean** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] [default to null] |

### Return type

[**QueryRangeResponse**](../Models/QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="queryRangeMany"></a>

# **queryRangeMany**

> QueryRangeManyResponse queryRangeMany(indices, lat, lng, range, count, sorted)

Search multiple indices for objects nearby

    Search geospatial many indices for all keys within some distance

### Parameters

| Name        | Type                            | Description                                                                          | Notes                        |
| ----------- | ------------------------------- | ------------------------------------------------------------------------------------ | ---------------------------- |
| **indices** | [**List**](../Models/String.md) | List of indices to search                                                            | [default to null]            |
| **lat**     | **Double**                      | Latitude                                                                             | [default to null]            |
| **lng**     | **Double**                      | Longitude                                                                            | [default to null]            |
| **range**   | **Integer**                     | Search radius in kilometers                                                          | [default to null]            |
| **count**   | **Integer**                     | Maximum number of neighbors that can be returned (default 100)                       | [optional] [default to null] |
| **sorted**  | **Boolean**                     | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] [default to null] |

### Return type

[**QueryRangeManyResponse**](../Models/QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="removeKey"></a>

# **removeKey**

> RemoveKeyResponse removeKey(index, RemoveKey)

Remove key from index

    Removes key from geospatial index

### Parameters

| Name          | Type                                    | Description           | Notes             |
| ------------- | --------------------------------------- | --------------------- | ----------------- |
| **index**     | **String**                              | Geospatial index name | [default to null] |
| **RemoveKey** | [**RemoveKey**](../Models/RemoveKey.md) |                       |                   |

### Return type

[**RemoveKeyResponse**](../Models/RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

<a name="removeKeyBatch"></a>

# **removeKeyBatch**

> RemoveKeyBatchResponse removeKeyBatch(index, RemoveKeyBatch)

Remove multiple keys from index

    Removes multiple keys from geospatial index

### Parameters

| Name               | Type                                              | Description           | Notes             |
| ------------------ | ------------------------------------------------- | --------------------- | ----------------- |
| **index**          | **String**                                        | Geospatial index name | [default to null] |
| **RemoveKeyBatch** | [**RemoveKeyBatch**](../Models/RemoveKeyBatch.md) |                       |                   |

### Return type

[**RemoveKeyBatchResponse**](../Models/RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json
