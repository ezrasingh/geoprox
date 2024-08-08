# GeoproxClient::InsertKeyBatchResponse

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **errors** | **Hash&lt;String, String&gt;** | Contains information about which keys failed to be inserted and the associated error details. |  |
| **results** | **Hash&lt;String, String&gt;** | Object keys that have been inserted in the index and their geohash. |  |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::InsertKeyBatchResponse.new(
  errors: null,
  results: null
)
```

