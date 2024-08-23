# GeoproxClient::InsertKey

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **key** | **String** | Object key |  |
| **lat** | **Float** | Latitude |  |
| **lng** | **Float** | Longitude |  |
| **ttl** | **Integer** | The time-to-live (TTL) for this key, in seconds | [optional] |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::InsertKey.new(
  key: null,
  lat: null,
  lng: null,
  ttl: null
)
```

