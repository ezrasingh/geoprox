# GeoproxClient::GeoshardApiApi

All URIs are relative to *http://localhost*

| Method | HTTP request | Description |
| ------ | ------------ | ----------- |
| [**create_index**](GeoshardApiApi.md#create_index) | **POST** /api/v1/shard/{index} | Create geospatial index |
| [**drop_index**](GeoshardApiApi.md#drop_index) | **DELETE** /api/v1/shard/{index} | Deletes geospatial index |
| [**insert_key**](GeoshardApiApi.md#insert_key) | **PUT** /api/v1/shard/{index} | Insert key into index |
| [**insert_key_batch**](GeoshardApiApi.md#insert_key_batch) | **PUT** /api/v1/shard/{index}/batch | Insert multiple keys into index |
| [**query_range**](GeoshardApiApi.md#query_range) | **GET** /api/v1/shard/{index} | Search index for objects nearby |
| [**query_range_many**](GeoshardApiApi.md#query_range_many) | **GET** /api/v1/shard | Search multiple indices for objects nearby |
| [**remove_key**](GeoshardApiApi.md#remove_key) | **PATCH** /api/v1/shard/{index} | Remove key from index |
| [**remove_key_batch**](GeoshardApiApi.md#remove_key_batch) | **PATCH** /api/v1/shard/{index}/batch | Remove multiple keys from index |


## create_index

> <CreateIndexResponse> create_index(index)

Create geospatial index

Creates an in-memory index within this geoshard

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name

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
| **index** | **String** | Geospatial index name |  |

### Return type

[**CreateIndexResponse**](CreateIndexResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## drop_index

> <DropIndexResponse> drop_index(index)

Deletes geospatial index

Drop index. All keys will be lost

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name

begin
  # Deletes geospatial index
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
  # Deletes geospatial index
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
| **index** | **String** | Geospatial index name |  |

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
index = 'index_example' # String | Geospatial index name
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
| **index** | **String** | Geospatial index name |  |
| **insert_key** | [**InsertKey**](InsertKey.md) |  |  |

### Return type

[**InsertKeyResponse**](InsertKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## insert_key_batch

> <InsertKeyBatchResponse> insert_key_batch(index, insert_key_batch)

Insert multiple keys into index

Inserts multiple keys into geospatial index

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name
insert_key_batch = GeoproxClient::InsertKeyBatch.new({keys: [GeoproxClient::InsertKey.new({key: 'key_example', lat: 3.56, lng: 3.56})], preserve_order: false}) # InsertKeyBatch | 

begin
  # Insert multiple keys into index
  result = api_instance.insert_key_batch(index, insert_key_batch)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->insert_key_batch: #{e}"
end
```

#### Using the insert_key_batch_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<InsertKeyBatchResponse>, Integer, Hash)> insert_key_batch_with_http_info(index, insert_key_batch)

```ruby
begin
  # Insert multiple keys into index
  data, status_code, headers = api_instance.insert_key_batch_with_http_info(index, insert_key_batch)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <InsertKeyBatchResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->insert_key_batch_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** | Geospatial index name |  |
| **insert_key_batch** | [**InsertKeyBatch**](InsertKeyBatch.md) |  |  |

### Return type

[**InsertKeyBatchResponse**](InsertKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## query_range

> <QueryRangeResponse> query_range(index, lat, lng, range, opts)

Search index for objects nearby

Search geospatial index for all keys within some distance

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name
lat = 1.2 # Float | Latitude
lng = 1.2 # Float | Longitude
range = 56 # Integer | Search radius in kilometers
opts = {
  count: 56, # Integer | Maximum number of neighbors that can be returned (default 100)
  sorted: true # Boolean | If enabled neighbors will be sorted by distance, nearest to furthest (default false)
}

begin
  # Search index for objects nearby
  result = api_instance.query_range(index, lat, lng, range, opts)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->query_range: #{e}"
end
```

#### Using the query_range_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<QueryRangeResponse>, Integer, Hash)> query_range_with_http_info(index, lat, lng, range, opts)

```ruby
begin
  # Search index for objects nearby
  data, status_code, headers = api_instance.query_range_with_http_info(index, lat, lng, range, opts)
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
| **index** | **String** | Geospatial index name |  |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **range** | **Integer** | Search radius in kilometers |  |
| **count** | **Integer** | Maximum number of neighbors that can be returned (default 100) | [optional] |
| **sorted** | **Boolean** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**QueryRangeResponse**](QueryRangeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## query_range_many

> <QueryRangeManyResponse> query_range_many(indices, lat, lng, range, opts)

Search multiple indices for objects nearby

Search geospatial many indices for all keys within some distance

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
indices = ['inner_example'] # Array<String> | List of indices to search
lat = 1.2 # Float | Latitude
lng = 1.2 # Float | Longitude
range = 56 # Integer | Search radius in kilometers
opts = {
  count: 56, # Integer | Maximum number of neighbors that can be returned (default 100)
  sorted: true # Boolean | If enabled neighbors will be sorted by distance, nearest to furthest (default false)
}

begin
  # Search multiple indices for objects nearby
  result = api_instance.query_range_many(indices, lat, lng, range, opts)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->query_range_many: #{e}"
end
```

#### Using the query_range_many_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<QueryRangeManyResponse>, Integer, Hash)> query_range_many_with_http_info(indices, lat, lng, range, opts)

```ruby
begin
  # Search multiple indices for objects nearby
  data, status_code, headers = api_instance.query_range_many_with_http_info(indices, lat, lng, range, opts)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <QueryRangeManyResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->query_range_many_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **indices** | [**Array&lt;String&gt;**](String.md) | List of indices to search |  |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **range** | **Integer** | Search radius in kilometers |  |
| **count** | **Integer** | Maximum number of neighbors that can be returned (default 100) | [optional] |
| **sorted** | **Boolean** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

### Return type

[**QueryRangeManyResponse**](QueryRangeManyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json


## remove_key

> <RemoveKeyResponse> remove_key(index, remove_key)

Remove key from index

Removes key from geospatial index

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name
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

> <Array(<RemoveKeyResponse>, Integer, Hash)> remove_key_with_http_info(index, remove_key)

```ruby
begin
  # Remove key from index
  data, status_code, headers = api_instance.remove_key_with_http_info(index, remove_key)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <RemoveKeyResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->remove_key_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** | Geospatial index name |  |
| **remove_key** | [**RemoveKey**](RemoveKey.md) |  |  |

### Return type

[**RemoveKeyResponse**](RemoveKeyResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json


## remove_key_batch

> <RemoveKeyBatchResponse> remove_key_batch(index, remove_key_batch)

Remove multiple keys from index

Removes multiple keys from geospatial index

### Examples

```ruby
require 'time'
require 'geoprox_client'

api_instance = GeoproxClient::GeoshardApiApi.new
index = 'index_example' # String | Geospatial index name
remove_key_batch = GeoproxClient::RemoveKeyBatch.new({keys: ['keys_example']}) # RemoveKeyBatch | 

begin
  # Remove multiple keys from index
  result = api_instance.remove_key_batch(index, remove_key_batch)
  p result
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->remove_key_batch: #{e}"
end
```

#### Using the remove_key_batch_with_http_info variant

This returns an Array which contains the response data, status code and headers.

> <Array(<RemoveKeyBatchResponse>, Integer, Hash)> remove_key_batch_with_http_info(index, remove_key_batch)

```ruby
begin
  # Remove multiple keys from index
  data, status_code, headers = api_instance.remove_key_batch_with_http_info(index, remove_key_batch)
  p status_code # => 2xx
  p headers # => { ... }
  p data # => <RemoveKeyBatchResponse>
rescue GeoproxClient::ApiError => e
  puts "Error when calling GeoshardApiApi->remove_key_batch_with_http_info: #{e}"
end
```

### Parameters

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **index** | **String** | Geospatial index name |  |
| **remove_key_batch** | [**RemoveKeyBatch**](RemoveKeyBatch.md) |  |  |

### Return type

[**RemoveKeyBatchResponse**](RemoveKeyBatchResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

