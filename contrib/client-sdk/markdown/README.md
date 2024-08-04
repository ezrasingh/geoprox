# Documentation for geoprox-server

<a name="documentation-for-api-endpoints"></a>
## Documentation for API Endpoints

All URIs are relative to *http://localhost*

| Class | Method | HTTP request | Description |
|------------ | ------------- | ------------- | -------------|
| *GeohashApiApi* | [**decodeGeohash**](Apis/GeohashApiApi.md#decodegeohash) | **GET** /api/v1/geohash/{ghash} | Decode geohash into coordinates. |
*GeohashApiApi* | [**encodeLatlng**](Apis/GeohashApiApi.md#encodelatlng) | **GET** /api/v1/geohash | Encode coordinates into geohash |
*GeohashApiApi* | [**getNeighbors**](Apis/GeohashApiApi.md#getneighbors) | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions |
| *GeoshardApiApi* | [**createIndex**](Apis/GeoshardApiApi.md#createindex) | **POST** /api/v1/shard/{index} | Create geospatial index |
*GeoshardApiApi* | [**dropIndex**](Apis/GeoshardApiApi.md#dropindex) | **DELETE** /api/v1/shard/{index} | Drop index |
*GeoshardApiApi* | [**insertKey**](Apis/GeoshardApiApi.md#insertkey) | **PUT** /api/v1/shard/{index} | Insert key into index |
*GeoshardApiApi* | [**queryRange**](Apis/GeoshardApiApi.md#queryrange) | **GET** /api/v1/shard/{index} | Search nearby |
*GeoshardApiApi* | [**removeKey**](Apis/GeoshardApiApi.md#removekey) | **PATCH** /api/v1/shard/{index} | Remove key from index |


<a name="documentation-for-models"></a>
## Documentation for Models

 - [CreateIndexResponse](./Models/CreateIndexResponse.md)
 - [DecodeGeohashResponse](./Models/DecodeGeohashResponse.md)
 - [DropIndexResponse](./Models/DropIndexResponse.md)
 - [EncodeLatLng](./Models/EncodeLatLng.md)
 - [EncodeLatLngResponse](./Models/EncodeLatLngResponse.md)
 - [GeohashNeighborsResponse](./Models/GeohashNeighborsResponse.md)
 - [InsertKey](./Models/InsertKey.md)
 - [InsertKeyResponse](./Models/InsertKeyResponse.md)
 - [Neighbor](./Models/Neighbor.md)
 - [QueryRange](./Models/QueryRange.md)
 - [QueryRangeResponse](./Models/QueryRangeResponse.md)
 - [RemoveKey](./Models/RemoveKey.md)
 - [RemoveKeyResponse](./Models/RemoveKeyResponse.md)


<a name="documentation-for-authorization"></a>
## Documentation for Authorization

All endpoints do not require authorization.
