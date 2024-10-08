# Geoprox Server

Geoprox Server is an in-memory geospatial search engine built on top of [Geoprox Core](https://crates.io/crates/geoprox-core/).

It provides an API for encoding and decoding geohashes, querying neighboring geohashes, and conducting efficient geospatial searches within specific ranges. This makes it a practical tool for managing and querying location-based data with speed and accuracy.

This service is well-suited for real-time applications such as ride-sharing and food delivery services, where quick and accurate location tracking is crucial.

> **Need an API Client?** See, [`contrib/client-sdk`](https://github.com/ezrasingh/geoprox/tree/main/contrib/client-sdk/) for available HTTP client libraries or generate your own.

- Swagger UI is available at the `/swagger-ui/` endpoint.
- The OpenAPI specification is accessible at the `/api-spec/openapi.json` endpoint.

## API Endpoints

| Api           | Method                                                   | HTTP request                              | Description                                |
| ------------- | -------------------------------------------------------- | ----------------------------------------- | ------------------------------------------ |
| `GeohashApi`  | [**decodeGeohash**](Apis/GeohashApi.md#decodegeohash)    | **GET** /api/v1/geohash/{ghash}           | Decode geohash into coordinates.           |
| `GeohashApi`  | [**encodeLatlng**](Apis/GeohashApi.md#encodelatlng)      | **GET** /api/v1/geohash                   | Encode coordinates into geohash            |
| `GeohashApi`  | [**getNeighbors**](Apis/GeohashApi.md#getneighbors)      | **GET** /api/v1/geohash/{ghash}/neighbors | Neighboring regions                        |
| `GeoshardApi` | [**createIndex**](Apis/GeoshardApi.md#createindex)       | **POST** /api/v1/shard/{index}            | Create geospatial index                    |
| `GeoshardApi` | [**dropIndex**](Apis/GeoshardApi.md#dropindex)           | **DELETE** /api/v1/shard/{index}          | Deletes geospatial index                   |
| `GeoshardApi` | [**insertKey**](Apis/GeoshardApi.md#insertkey)           | **PUT** /api/v1/shard/{index}             | Insert key into index                      |
| `GeoshardApi` | [**insertKeyBatch**](Apis/GeoshardApi.md#insertkeybatch) | **PUT** /api/v1/shard/{index}/batch       | Insert multiple keys into index            |
| `GeoshardApi` | [**queryRange**](Apis/GeoshardApi.md#queryrange)         | **GET** /api/v1/shard/{index}             | Search index for objects nearby            |
| `GeoshardApi` | [**queryRangeMany**](Apis/GeoshardApi.md#queryrangemany) | **GET** /api/v1/shard                     | Search multiple indices for objects nearby |
| `GeoshardApi` | [**removeKey**](Apis/GeoshardApi.md#removekey)           | **PATCH** /api/v1/shard/{index}           | Remove key from index                      |
| `GeoshardApi` | [**removeKeyBatch**](Apis/GeoshardApi.md#removekeybatch) | **PATCH** /api/v1/shard/{index}/batch     | Remove multiple keys from index            |

Check out the [API documentation](https://github.com/ezrasingh/geoprox/tree/main/geoprox-server/docs/) for detailed descriptions of all the endpoints.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](https://github.com/ezrasingh/geoprox/tree/main/CONTRIBUTING.md) for guidelines.

## License

This project is licensed under the [Apache 2.0](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-APACHE) or [MIT License](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-MIT) (your choice).
