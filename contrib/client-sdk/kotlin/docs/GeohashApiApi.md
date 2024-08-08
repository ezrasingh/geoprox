# GeohashApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
| ------------- | ------------- | ------------- |
| [**decodeGeohash**](GeohashApiApi.md#decodeGeohash) | **GET** /api/v1/geohash/{ghash}/ | Decode geohash into coordinates. |
| [**encodeLatlng**](GeohashApiApi.md#encodeLatlng) | **GET** /api/v1/geohash/ | Encode coordinates into geohash |
| [**getNeighbors**](GeohashApiApi.md#getNeighbors) | **GET** /api/v1/geohash/{ghash}/neighbors/ | Neighboring regions |


<a id="decodeGeohash"></a>
# **decodeGeohash**
> DecodeGeohashResponse decodeGeohash(ghash)

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeohashApiApi()
val ghash : kotlin.String = ghash_example // kotlin.String | Geohash encoded region
try {
    val result : DecodeGeohashResponse = apiInstance.decodeGeohash(ghash)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeohashApiApi#decodeGeohash")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeohashApiApi#decodeGeohash")
    e.printStackTrace()
}
```

### Parameters
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **ghash** | **kotlin.String**| Geohash encoded region | |

### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="encodeLatlng"></a>
# **encodeLatlng**
> EncodeLatLngResponse encodeLatlng(lat, lng, depth)

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeohashApiApi()
val lat : kotlin.Double = 1.2 // kotlin.Double | Latitude
val lng : kotlin.Double = 1.2 // kotlin.Double | Longitude
val depth : kotlin.Int = 56 // kotlin.Int | Determines geohash length
try {
    val result : EncodeLatLngResponse = apiInstance.encodeLatlng(lat, lng, depth)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeohashApiApi#encodeLatlng")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeohashApiApi#encodeLatlng")
    e.printStackTrace()
}
```

### Parameters
| **lat** | **kotlin.Double**| Latitude | |
| **lng** | **kotlin.Double**| Longitude | |
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **depth** | **kotlin.Int**| Determines geohash length | |

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

<a id="getNeighbors"></a>
# **getNeighbors**
> GeohashNeighborsResponse getNeighbors(ghash)

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Example
```kotlin
// Import classes:
//import org.geoprox.client.infrastructure.*
//import org.geoprox.client.models.*

val apiInstance = GeohashApiApi()
val ghash : kotlin.String = ghash_example // kotlin.String | Geohash encoded region
try {
    val result : GeohashNeighborsResponse = apiInstance.getNeighbors(ghash)
    println(result)
} catch (e: ClientException) {
    println("4xx response calling GeohashApiApi#getNeighbors")
    e.printStackTrace()
} catch (e: ServerException) {
    println("5xx response calling GeohashApiApi#getNeighbors")
    e.printStackTrace()
}
```

### Parameters
| Name | Type | Description  | Notes |
| ------------- | ------------- | ------------- | ------------- |
| **ghash** | **kotlin.String**| Geohash encoded region | |

### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json

