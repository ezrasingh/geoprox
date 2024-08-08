# Documentation for geoprox-server

<a name="documentation-for-api-endpoints"></a>

## Documentation for API Endpoints

All URIs are relative to _http://localhost_

| Api           | Method                                                   | HTTP request                               | Description                                |
| ------------- | -------------------------------------------------------- | ------------------------------------------ | ------------------------------------------ |
| `GeohashApi`  | [**decodeGeohash**](Apis/GeohashApi.md#decodegeohash)    | **GET** /api/v1/geohash/{ghash}/           | Decode geohash into coordinates.           |
| `GeohashApi`  | [**encodeLatlng**](Apis/GeohashApi.md#encodelatlng)      | **GET** /api/v1/geohash/                   | Encode coordinates into geohash            |
| `GeohashApi`  | [**getNeighbors**](Apis/GeohashApi.md#getneighbors)      | **GET** /api/v1/geohash/{ghash}/neighbors/ | Neighboring regions                        |
| `GeoshardApi` | [**createIndex**](Apis/GeoshardApi.md#createindex)       | **POST** /api/v1/shard/{index}/            | Create geospatial index                    |
| `GeoshardApi` | [**dropIndex**](Apis/GeoshardApi.md#dropindex)           | **DELETE** /api/v1/shard/{index}/          | Deletes geospatial index                   |
| `GeoshardApi` | [**insertKey**](Apis/GeoshardApi.md#insertkey)           | **PUT** /api/v1/shard/{index}/             | Insert key into index                      |
| `GeoshardApi` | [**insertKeyBatch**](Apis/GeoshardApi.md#insertkeybatch) | **PUT** /api/v1/shard/{index}/batch/       | Insert multiple keys into index            |
| `GeoshardApi` | [**queryRange**](Apis/GeoshardApi.md#queryrange)         | **GET** /api/v1/shard/{index}/             | Search index for objects nearby            |
| `GeoshardApi` | [**queryRangeMany**](Apis/GeoshardApi.md#queryrangemany) | **GET** /api/v1/shard/                     | Search multiple indices for objects nearby |
| `GeoshardApi` | [**removeKey**](Apis/GeoshardApi.md#removekey)           | **PATCH** /api/v1/shard/{index}/           | Remove key from index                      |
| `GeoshardApi` | [**removeKeyBatch**](Apis/GeoshardApi.md#removekeybatch) | **PATCH** /api/v1/shard/{index}/batch/     | Remove multiple keys from index            |

<a name="documentation-for-models"></a>

## Documentation for Models

- [CreateIndexResponse](./Models/CreateIndexResponse.md)
- [DecodeGeohashResponse](./Models/DecodeGeohashResponse.md)
- [DropIndexResponse](./Models/DropIndexResponse.md)
- [EncodeLatLng](./Models/EncodeLatLng.md)
- [EncodeLatLngResponse](./Models/EncodeLatLngResponse.md)
- [GeohashNeighborsResponse](./Models/GeohashNeighborsResponse.md)
- [InsertKey](./Models/InsertKey.md)
- [InsertKeyBatch](./Models/InsertKeyBatch.md)
- [InsertKeyBatchResponse](./Models/InsertKeyBatchResponse.md)
- [InsertKeyResponse](./Models/InsertKeyResponse.md)
- [Neighbor](./Models/Neighbor.md)
- [QueryRange](./Models/QueryRange.md)
- [QueryRangeMany](./Models/QueryRangeMany.md)
- [QueryRangeManyResponse](./Models/QueryRangeManyResponse.md)
- [QueryRangeResponse](./Models/QueryRangeResponse.md)
- [RemoveKey](./Models/RemoveKey.md)
- [RemoveKeyBatch](./Models/RemoveKeyBatch.md)
- [RemoveKeyBatchResponse](./Models/RemoveKeyBatchResponse.md)
- [RemoveKeyResponse](./Models/RemoveKeyResponse.md)

<a name="documentation-for-authorization"></a>

## Documentation for Authorization

All endpoints do not require authorization.
