# GeoproxClient::GeoshardApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
| ------ | ------------ | ----------- |
| [**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index} | Create geospatial index |
| [**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index} | Drop index |
| [**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index} | Insert key into index |
| [**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index} | Search nearby |
| [**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index} | Remove key from index |


## create_index

> <CreateIndexResponse> create_index(index)

Create geospatial index

Creates an in-memory index within this geoshard

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | 

begin
  # Create geospatial index
  result = api_instance.create_index(index)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->create_index: #{e}"
end
```

#### Using the create_index_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<CreateIndexResponse>, Integer, Hash)> create_index_with_http_info(index)

```ruby
begin
  # Create geospatial index
  data, status_code, headers = api_instance.create_index_with_http_info(index)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <CreateIndexResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->create_index_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** |  |  |

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## drop_index

> <DropIndexResponse> drop_index(index)

Drop index

Deletes geospatial index, all keys will be lost

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | 

begin
  # Drop index
  result = api_instance.drop_index(index)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->drop_index: #{e}"
end
```

#### Using the drop_index_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<DropIndexResponse>, Integer, Hash)> drop_index_with_http_info(index)

```ruby
begin
  # Drop index
  data, status_code, headers = api_instance.drop_index_with_http_info(index)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <DropIndexResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->drop_index_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** |  |  |

### Return type

[**DropIndexResponse**](DropIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## insert_key

> <InsertKeyResponse> insert_key(index, insert_key)

Insert key into index

Inserts key into geospatial index

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | 
insert_key = GeoproxClient::InsertKey.new({key: 'key_example', lat: 3.56, lng: 3.56}) # InsertKey | 

begin
  # Insert key into index
  result = api_instance.insert_key(index, insert_key)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->insert_key: #{e}"
end
```

#### Using the insert_key_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<InsertKeyResponse>, Integer, Hash)> insert_key_with_http_info(index, insert_key)

```ruby
begin
  # Insert key into index
  data, status_code, headers = api_instance.insert_key_with_http_info(index, insert_key)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <InsertKeyResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->insert_key_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** |  |  |
| **insert_key** | [**InsertKey**](InsertKey.md) |  |  |

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## query_range

> <QueryRangeResponse> query_range(lat, lng, range, index)

Search nearby

Search geospatial index for all keys within some distance

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
lat = 1.2 # Float | latitude
lng = 1.2 # Float | longitude
range = 56 # Integer | search radius in kilometers
index = 'index_example' # String | 

begin
  # Search nearby
  result = api_instance.query_range(lat, lng, range, index)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->query_range: #{e}"
end
```

#### Using the query_range_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<QueryRangeResponse>, Integer, Hash)> query_range_with_http_info(lat, lng, range, index)

```ruby
begin
  # Search nearby
  data, status_code, headers = api_instance.query_range_with_http_info(lat, lng, range, index)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <QueryRangeResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->query_range_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **lat** | **Float** | latitude |  |
| **lng** | **Float** | longitude |  |
| **range** | **Integer** | search radius in kilometers |  |
| **index** | **String** |  |  |

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## remove_key

> <InsertKeyResponse> remove_key(index, remove_key)

Remove key from index

Removed key from geospatial index

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | 
remove_key = GeoproxClient::RemoveKey.new({key: 'key_example'}) # RemoveKey | 

begin
  # Remove key from index
  result = api_instance.remove_key(index, remove_key)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->remove_key: #{e}"
end
```

#### Using the remove_key_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<InsertKeyResponse>, Integer, Hash)> remove_key_with_http_info(index, remove_key)

```ruby
begin
  # Remove key from index
  data, status_code, headers = api_instance.remove_key_with_http_info(index, remove_key)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <InsertKeyResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->remove_key_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** |  |  |
| **remove_key** | [**RemoveKey**](RemoveKey.md) |  |  |

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

