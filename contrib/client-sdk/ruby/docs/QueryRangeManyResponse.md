# GeoproxClient::QueryRangeManyResponse

## Properties

| Name | Type | Description | Notes |
| ---- | ---- | ----------- | ----- |
| **errors** | **Hash&lt;String, String&gt;** | Contains information about any errors occured during batch search. |  |
| **results** | **Hash&lt;String, Array&lt;Neighbor&gt;&gt;** | Object keys found within range |  |

## Example

```ruby
require 'geoprox_client'

instance = GeoproxClient::QueryRangeManyResponse.new(
  errors: null,
  results: null
)
```

