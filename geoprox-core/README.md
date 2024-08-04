# Geoprox Core

A Rust crate designed for geospatial indexing and sharding. It includes two primary modules: `cache` and `shard`. These modules facilitate efficient geospatial queries and indexing for applications such as food delivery services, real-time inventory tracking, and more.

> **Looking for an API implementation?** See, [`geoprox-server`](https://crates.io/crates/geoprox-server/) for the HTTP API version of this service and [`contrib/client-sdk`](https://github.com/ezrasingh/geoprox/tree/main/contrib/client-sdk) for HTTP client libraries.

## Features

- **Geospatial Indexing**: Efficiently index and search for resources based on their geographic location.
- **Geospatial Sharding**: Manage and query distributed datasets with geospatial awareness.

## Modules

### SpatialIndex

The `cache` module, implemented as `SpatialIndex`, allows for placing and searching resources based on their geohash-encoded locations. Key features include:

- **Placing Resources**: Add resources to the index with their geohash-encoded locations.
- **Searching Resources**: Perform range queries to find resources within a specified distance from a given location.

#### Example Usage

```rust
extern crate geoprox_core;

let mut geo_index = geoprox_core::SpatialIndex::default();

// ? place elements into index
geo_index.place_resource("player1", &geohash::encode([40.7129, 74.007].into(), depth).unwrap());
geo_index.place_resource("player2", &geohash::encode([40.7127, 74.005].into(), depth).unwrap());

// ? search index for elements near New York
let nearby: LatLngCoord = [40.7128, 74.006];
let within: f64 = 200.0; // 200km radius
let count = 100; // return up to 100 results
let sorted = true; // sort results by distance
let depth = 6; // ? determines geohash length (i.e precision)
let res = geo_index.search(nearby, range, count, sorted, Some(depth)).unwrap();

assert_eq!(res.len(), 2);
res.iter().for_each(|neighbor| {
    assert!(neighbor.distance <= range);
});
```

### GeoShard

The `shard` module, implemented as `GeoShard`, provides a mechanism for sharding datasets geographically and performing range queries. Key features include:

- **Creating Indexes**: Initialize geospatial indexes for different datasets.
- **Inserting Keys**: Insert keys into the geospatial index with their corresponding locations.
- **Querying Ranges**: Query the index to find keys within a specified range from a given location.
- **Drop Index**: Deletes the index and its associated keys.

#### Example Usage

```rust
extern crate geoprox_core;

let mut shard = geoprox_core::GeoShard::default();
shard.create_index("drivers").unwrap();

// ? place drivers in index
shard.insert_key("drivers", "alice", [36.2049, 138.253]).unwrap();
shard.insert_key("drivers", "bob", [36.2047, 138.2528]).unwrap();

// ? search 'drivers' near Japan
let nearby: LatLngCoord = [36.2048, 138.2529];
let within: f64 = 50.0; // 50km radius
let count = 100; // return up to 100 results
let sorted = true; // sort results by distance
let res = shard.query_range("drivers", nearby, within, count, sorted).unwrap();
println!("found: {:#?}", res);
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
