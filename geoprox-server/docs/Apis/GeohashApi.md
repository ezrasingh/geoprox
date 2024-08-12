# GeohashApiApi

All URIs are relative to _http://localhost_

| Method                                           | HTTP request                              | Description                      |
| ------------------------------------------------ | ----------------------------------------- | -------------------------------- |
| [**decodeGeohash**](GeohashApi.md#decodeGeohash) | **GET** /api/v1/geohash/{ghash}           | Decode geohash into coordinates. |
| [**encodeLatlng**](GeohashApi.md#encodeLatlng)   | **GET** /api/v1/geohash                   | Encode coordinates into geohash  |
| [**getNeighbors**](GeohashApi.md#getNeighbors)   | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions              |

<a name="decodeGeohash"></a>

# **decodeGeohash**

> DecodeGeohashResponse decodeGeohash(ghash)

Decode geohash into coordinates.

    Decode geohash by path param, returns coordinates with precision estimates.

### Parameters

| Name      | Type       | Description            | Notes             |
| --------- | ---------- | ---------------------- | ----------------- |
| **ghash** | **String** | Geohash encoded region | [default to null] |

### Return type

[**DecodeGeohashResponse**](../Models/DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="encodeLatlng"></a>

# **encodeLatlng**

> EncodeLatLngResponse encodeLatlng(lat, lng, depth)

Encode coordinates into geohash

    Encode coordinates by query params, returns geohash.

### Parameters

| Name      | Type        | Description               | Notes             |
| --------- | ----------- | ------------------------- | ----------------- |
| **lat**   | **Double**  | Latitude                  | [default to null] |
| **lng**   | **Double**  | Longitude                 | [default to null] |
| **depth** | **Integer** | Determines geohash length | [default to null] |

### Return type

[**EncodeLatLngResponse**](../Models/EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

<a name="getNeighbors"></a>

# **getNeighbors**

> GeohashNeighborsResponse getNeighbors(ghash)

Neighboring regions

    Returns geohash neighbors in all cardinal directions.

### Parameters

| Name      | Type       | Description            | Notes             |
| --------- | ---------- | ---------------------- | ----------------- |
| **ghash** | **String** | Geohash encoded region | [default to null] |

### Return type

[**GeohashNeighborsResponse**](../Models/GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json
