# Geoprox Core

Geoprox Core is the foundational Rust crate for the [Geoprox](https://github.com/ezrasingh/geoprox/) project, providing robust geospatial indexing and sharding capabilities. It includes two primary modules:

- **`cache`**: Manages in-memory storage and retrieval of geospatial data to ensure quick access and efficient querying.
- **`shard`**: Handles the partitioning and indexing of geospatial information, optimizing data distribution and retrieval across large datasets.

These modules are built to handle various use cases, such as food delivery services and real-time inventory tracking.

> **Looking for an API implementation?** See, [`geoprox-server`](https://crates.io/crates/geoprox-server/) for the HTTP API version of this service and [`contrib/client-sdk`](https://github.com/ezrasingh/geoprox/tree/main/contrib/client-sdk) for HTTP client libraries.

## Features

- **Geospatial Indexing**: Efficiently index and search for resources based on their geographic location.
- **Geospatial Sharding**: Manage and query distributed datasets with geospatial awareness.

## Data Stuctures

### SpatialIndex

The `cache` module implements the `SpatialIndex` data structure, enabling efficient placement and searching of resources based on their geohash-encoded locations.

Key features include:

- **Placing Resources**: Add resources to the index with their geohash-encoded locations.
- **Searching Resources**: Perform range queries to find resources within a specified distance from a given location.

#### Example Usage

```rust
extern crate geoprox_core;

let mut geo_index = geoprox_core::SpatialIndex::default();

// ? place object keys into index
geo_index.insert("alice", "s00j8n0");
geo_index.insert("bob", "s00j8n1");

// ? search index for objects near New York
let nearby: LatLngCoord = [40.7128, 74.006];
let within: f64 = 200.0; // 200km radius
let count = 100; // return up to 100 results
let sorted = true; // sort results by distance
let search_depth = 6; // search precision (higher means more precise)
let res = geo_index.search(nearby, within, count, sorted, depth).unwrap();

println!("found: {:#?}", res);

assert_eq!(res.len(), 2);

res.iter().for_each(|neighbor| {
    assert!(neighbor.distance <= within);
});
```

### GeoShard

The `shard` module implements the `GeoShard` data structure, which facilitates geographical sharding of datasets and enables efficient range queries.

Key features include:

- **Creating Indexes**: Initialize geospatial indexes for different datasets.
- **Inserting Keys**: Insert keys into the geospatial index with their corresponding locations.
- **Querying Ranges**: Query the index to find keys within a specified range from a given location.
- **Drop Index**: Deletes the index and its associated keys.
- **Batch Commands**: Insert, Remove or Query objects in bulk.

#### Example Usage

```rust
extern crate geoprox_core;

let mut shard = geoprox_core::GeoShard::default();
// ? create an index to store driver coordinates
shard.create_index("drivers").unwrap();

// ? place drivers into index
let ttl = std::time::Duration::from_secs(10);
shard.insert_key(
    "drivers",
    "alice",
    [36.2049, 138.253],
    Some(ttl)
).unwrap();

shard.insert_key(
    "drivers",
    "bob",
    [36.2047, 138.2528],
    None
).unwrap();

// ? search drivers near Japan
let nearby: LatLngCoord = [36.2048, 138.2529];
let within: f64 = 50.0; // 50km radius
let count = 100; // return up to 100 results
let sorted = true; // sort results by distance

let res = shard.query_range("drivers", nearby, within, Some(count), Some(sorted));

println!("found: {:#?}", res.unwrap());
```

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-APACHE) or [MIT License](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-MIT) (your choice).
