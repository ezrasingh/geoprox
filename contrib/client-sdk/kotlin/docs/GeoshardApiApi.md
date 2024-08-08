# GeoshardApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**createIndex**](GeoshardApiApi.md#createIndex) | **POST** /api/v1/shard/{index}/ | Create geospatial index |
| [**dropIndex**](GeoshardApiApi.md#dropIndex) | **DELETE** /api/v1/shard/{index}/ | Deletes geospatial index |
| [**insertKey**](GeoshardApiApi.md#insertKey) | **PUT** /api/v1/shard/{index}/ | Insert key into index |
| [**insertKeyBatch**](GeoshardApiApi.md#insertKeyBatch) | **PUT** /api/v1/shard/{index}/batch/ | Insert multiple keys into index |
| [**queryRange**](GeoshardApiApi.md#queryRange) | **GET** /api/v1/shard/{index}/ | Search index for objects nearby |
| [**queryRangeMany**](GeoshardApiApi.md#queryRangeMany) | **GET** /api/v1/shard/ | Search multiple indices for objects nearby |
| [**removeKey**](GeoshardApiApi.md#removeKey) | **PATCH** /api/v1/shard/{index}/ | Remove key from index |
| [**removeKeyBatch**](GeoshardApiApi.md#removeKeyBatch) | **PATCH** /api/v1/shard/{index}/batch/ | Remove multiple keys from index |


<a id="createIndex"></a>
# **createIndex**
> CreateIndexResponse createIndex(index)

Create geospatial index

Creates an in-memory index within this geoshard

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
try {
    val result : CreateIndexResponse = apiInstance.createIndex(index)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#createIndex")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#createIndex")
    e.printStackTrace()
}
```

### Parameters
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **kotlin.String**| Geospatial index name | |

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="dropIndex"></a>
# **dropIndex**
> DropIndexResponse dropIndex(index)

Deletes geospatial index

Drop index. All keys will be lost

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
try {
    val result : DropIndexResponse = apiInstance.dropIndex(index)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#dropIndex")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#dropIndex")
    e.printStackTrace()
}
```

### Parameters
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **index** | **kotlin.String**| Geospatial index name | |

### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="insertKey"></a>
# **insertKey**
> InsertKeyResponse insertKey(index, insertKey)

Insert key into index

Inserts key into geospatial index

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
val insertKey : InsertKey =  // InsertKey | 
try {
    val result : InsertKeyResponse = apiInstance.insertKey(index, insertKey)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#insertKey")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#insertKey")
    e.printStackTrace()
}
```

### Parameters
| **index** | **kotlin.String**| Geospatial index name | |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **insertKey** | [**InsertKey**](InsertKey.md)|  | |

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a id="insertKeyBatch"></a>
# **insertKeyBatch**
> InsertKeyBatchResponse insertKeyBatch(index, insertKeyBatch)

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
val insertKeyBatch : InsertKeyBatch =  // InsertKeyBatch | 
try {
    val result : InsertKeyBatchResponse = apiInstance.insertKeyBatch(index, insertKeyBatch)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#insertKeyBatch")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#insertKeyBatch")
    e.printStackTrace()
}
```

### Parameters
| **index** | **kotlin.String**| Geospatial index name | |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **insertKeyBatch** | [**InsertKeyBatch**](InsertKeyBatch.md)|  | |

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a id="queryRange"></a>
# **queryRange**
> QueryRangeResponse queryRange(index, lat, lng, range, count, sorted)

Search index for objects nearby

Search geospatial index for all keys within some distance

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
val lat : kotlin.Double = 1.2 // kotlin.Double | Latitude
val lng : kotlin.Double = 1.2 // kotlin.Double | Longitude
val range : kotlin.Int = 56 // kotlin.Int | Search radius in kilometers
val count : kotlin.Int = 56 // kotlin.Int | Maximum number of neighbors that can be returned (default 100)
val sorted : kotlin.Boolean = true // kotlin.Boolean | If enabled neighbors will be sorted by distance, nearest to furthest (default false)
try {
    val result : QueryRangeResponse = apiInstance.queryRange(index, lat, lng, range, count, sorted)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#queryRange")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#queryRange")
    e.printStackTrace()
}
```

### Parameters
| **index** | **kotlin.String**| Geospatial index name | |
| **lat** | **kotlin.Double**| Latitude | |
| **lng** | **kotlin.Double**| Longitude | |
| **range** | **kotlin.Int**| Search radius in kilometers | |
| **count** | **kotlin.Int**| Maximum number of neighbors that can be returned (default 100) | [optional] |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **sorted** | **kotlin.Boolean**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="queryRangeMany"></a>
# **queryRangeMany**
> QueryRangeManyResponse queryRangeMany(indices, lat, lng, range, count, sorted)

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val indices : kotlin.collections.List<kotlin.String> =  // kotlin.collections.List<kotlin.String> | List of indices to search
val lat : kotlin.Double = 1.2 // kotlin.Double | Latitude
val lng : kotlin.Double = 1.2 // kotlin.Double | Longitude
val range : kotlin.Int = 56 // kotlin.Int | Search radius in kilometers
val count : kotlin.Int = 56 // kotlin.Int | Maximum number of neighbors that can be returned (default 100)
val sorted : kotlin.Boolean = true // kotlin.Boolean | If enabled neighbors will be sorted by distance, nearest to furthest (default false)
try {
    val result : QueryRangeManyResponse = apiInstance.queryRangeMany(indices, lat, lng, range, count, sorted)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#queryRangeMany")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#queryRangeMany")
    e.printStackTrace()
}
```

### Parameters
| **indices** | [**kotlin.collections.List&lt;kotlin.String&gt;**](kotlin.String.md)| List of indices to search | |
| **lat** | **kotlin.Double**| Latitude | |
| **lng** | **kotlin.Double**| Longitude | |
| **range** | **kotlin.Int**| Search radius in kilometers | |
| **count** | **kotlin.Int**| Maximum number of neighbors that can be returned (default 100) | [optional] |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **sorted** | **kotlin.Boolean**| If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="removeKey"></a>
# **removeKey**
> RemoveKeyResponse removeKey(index, removeKey)

Remove key from index

Removes key from geospatial index

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
val removeKey : RemoveKey =  // RemoveKey | 
try {
    val result : RemoveKeyResponse = apiInstance.removeKey(index, removeKey)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#removeKey")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#removeKey")
    e.printStackTrace()
}
```

### Parameters
| **index** | **kotlin.String**| Geospatial index name | |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **removeKey** | [**RemoveKey**](RemoveKey.md)|  | |

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

<a id="removeKeyBatch"></a>
# **removeKeyBatch**
> RemoveKeyBatchResponse removeKeyBatch(index, removeKeyBatch)

Remove multiple keys from index

Removes multiple keys from geospatial index

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeoshardApiApi()
val index : kotlin.String = index_example // kotlin.String | Geospatial index name
val removeKeyBatch : RemoveKeyBatch =  // RemoveKeyBatch | 
try {
    val result : RemoveKeyBatchResponse = apiInstance.removeKeyBatch(index, removeKeyBatch)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeoshardApiApi#removeKeyBatch")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeoshardApiApi#removeKeyBatch")
    e.printStackTrace()
}
```

### Parameters
| **index** | **kotlin.String**| Geospatial index name | |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **removeKeyBatch** | [**RemoveKeyBatch**](RemoveKeyBatch.md)|  | |

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json

