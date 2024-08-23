# GeoproxClient::InsertKeyBatch

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **keys** | [**Array&lt;InsertKey&gt;**](InsertKey.md) | Object key |  |
| **preserve_order** | **Boolean** |  |  |
| **ttl** | **Integer** | The time-to-live (TTL) for these keys, in seconds | [optional] |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::InsertKeyBatch.new(
  keys: null,
  preserve_order: null,
  ttl: null
)
```

