# GeoproxClient::QueryRange

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **count** | **Integer** | Maximum number of neighbors that can be returned (default 100) | [optional] |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **range** | **Integer** | Search radius in kilometers |  |
| **sorted** | **Boolean** | If enabled neighbors will be sorted by distance, nearest to furthest (default false) | [optional] |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::QueryRange.new(
  count: null,
  lat: null,
  lng: null,
  range: null,
  sorted: null
)
```

