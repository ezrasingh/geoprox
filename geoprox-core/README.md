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
let depth = 6; // ? determines geohash length (i.e precision)

geo_index.place_resource("alice", &geohash::encode([1.0, 0.0].into(), depth).unwrap());
geo_index.place_resource("bob", &geohash::encode([1.0, 1.0].into(), depth).unwrap());

let range = 1.0; // radius in kilometers
let origin = [0f64, 0f64]; // lat/lng
let res = geo_index.search(origin, &range, None).unwrap();

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

shard.insert_key("drivers", "alice", &[-0.25, 1.0]).unwrap();
shard.insert_key("drivers", "bob", &[1.0, 0.5]).unwrap();

// ? search 'drivers' within 150km radius near (1, 0.5)
let res = shard.query_range("drivers", [1.0, 0.5], &150.0).unwrap();
println!("found: {:#?}", res);
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
