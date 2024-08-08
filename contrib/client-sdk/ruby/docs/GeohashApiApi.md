# GeoproxClient::GeohashApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
| ------ | ------------ | ----------- |
| [**decode_geohash**](GeohashApiApi.md#decode_geohash) | **GET** /api/v1/geohash/{ghash}/ | Decode geohash into coordinates. |
| [**encode_latlng**](GeohashApiApi.md#encode_latlng) | **GET** /api/v1/geohash/ | Encode coordinates into geohash |
| [**get_neighbors**](GeohashApiApi.md#get_neighbors) | **GET** /api/v1/geohash/{ghash}/neighbors/ | Neighboring regions |


## decode_geohash

> <DecodeGeohashResponse> decode_geohash(ghash)

Decode geohash into coordinates.

Decode geohash by path param, returns coordinates with precision estimates.

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeohashApiApi.new
ghash = 'ghash_example' # String | Geohash encoded region

begin
  # Decode geohash into coordinates.
  result = api_instance.decode_geohash(ghash)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->decode_geohash: #{e}"
end
```

#### Using the decode_geohash_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<DecodeGeohashResponse>, Integer, Hash)> decode_geohash_with_http_info(ghash)

```ruby
begin
  # Decode geohash into coordinates.
  data, status_code, headers = api_instance.decode_geohash_with_http_info(ghash)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <DecodeGeohashResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->decode_geohash_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **ghash** | **String** | Geohash encoded region |  |

### Return type

[**DecodeGeohashResponse**](DecodeGeohashResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## encode_latlng

> <EncodeLatLngResponse> encode_latlng(lat, lng, depth)

Encode coordinates into geohash

Encode coordinates by query params, returns geohash.

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeohashApiApi.new
lat = 1.2 # Float | Latitude
lng = 1.2 # Float | Longitude
depth = 56 # Integer | Determines geohash length

begin
  # Encode coordinates into geohash
  result = api_instance.encode_latlng(lat, lng, depth)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->encode_latlng: #{e}"
end
```

#### Using the encode_latlng_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<EncodeLatLngResponse>, Integer, Hash)> encode_latlng_with_http_info(lat, lng, depth)

```ruby
begin
  # Encode coordinates into geohash
  data, status_code, headers = api_instance.encode_latlng_with_http_info(lat, lng, depth)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <EncodeLatLngResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->encode_latlng_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **depth** | **Integer** | Determines geohash length |  |

### Return type

[**EncodeLatLngResponse**](EncodeLatLngResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## get_neighbors

> <GeohashNeighborsResponse> get_neighbors(ghash)

Neighboring regions

Returns geohash neighbors in all cardinal directions.

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeohashApiApi.new
ghash = 'ghash_example' # String | Geohash encoded region

begin
  # Neighboring regions
  result = api_instance.get_neighbors(ghash)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->get_neighbors: #{e}"
end
```

#### Using the get_neighbors_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<GeohashNeighborsResponse>, Integer, Hash)> get_neighbors_with_http_info(ghash)

```ruby
begin
  # Neighboring regions
  data, status_code, headers = api_instance.get_neighbors_with_http_info(ghash)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <GeohashNeighborsResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeohashApiApi->get_neighbors_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **ghash** | **String** | Geohash encoded region |  |

### Return type

[**GeohashNeighborsResponse**](GeohashNeighborsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

