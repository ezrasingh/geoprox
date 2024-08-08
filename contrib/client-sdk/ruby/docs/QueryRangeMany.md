# GeoproxClient::QueryRangeMany

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **count** | **Integer** | Maximum number of neighbors that can be returned (default 100) | [optional] |
| **indices** | **Array&lt;String&gt;** | List of indices to search |  |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **range** | **Integer** | Search radius in kilometers |  |
| **sorted** | **Boolean** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::QueryRangeMany.new(
  count: null,
  indices: null,
  lat: null,
  lng: null,
  range: null,
  sorted: null
)
```

