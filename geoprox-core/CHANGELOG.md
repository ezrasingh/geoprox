# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.5.1

- Updated `kiddo` to [`v5.0.3`](https://github.com/sdd/kiddo/releases/tag/v5.0.3).

## 0.5.0

- Added support for key expirations ([#15](https://github.com/ezrasingh/geoprox/issues/15)).
- Implemented serde for `SpatialIndex` ([#17](https://github.com/ezrasingh/geoprox/issues/17)).
- renamed `cache.rs` to `index.rs`.
- Updated `kiddo` to [`v4.2.1`](https://github.com/sdd/kiddo/releases/tag/v4.2.1).

## 0.4.2

- Replaced the [`haversine`](https://crates.io/crates/haversine) crate with [`haversine-rs`](https://crates.io/crates/haversine-rs) after noticing it performed better in benchmarks.

## 0.4.1

- Moved configuration defaults into `GeoShard` ([#2](https://github.com/ezrasingh/geoprox/issues/2)).
- Remove [manual truncation in unsorted search](https://github.com/ezrasingh/geoprox/blob/f2074652e7b0e5eb8a4f0ae9bd4cb9f3c0c621df/geoprox-core/src/cache.rs#L175) by pinning Kiddo to latest commit ([#7](https://github.com/ezrasingh/geoprox/issues/7)) and handling edge case with `count` = 0.

## 0.4.0

- Added suport for batch insertion, query and removal ([#1](https://github.com/ezrasingh/geoprox/issues/1)).
- Improved error handling for `GeoShardError` by implementing `Display` and `Error`.
- Corrected issue with `(lat, lng)` ordering in `HaversineDistance` metric.

## 0.3.1

- Added additional tests for insertion, query and removal.
- Handled edge case when `count` eqauls 0.
- Handled bug in [Kiddo](https://github.com/sdd/kiddo/) where `count` is ignored when `sort` is disabled (see [Kiddo#168](https://github.com/sdd/kiddo/issues/168)).

## 0.3.0

- Using [`ahash`](https://crates.io/crates/ahash) as the internal hasher, since it out performed previous benchmarks.
- Swapped `std::collections` for [`hashbrown`](https://crates.io/crates/hashbrown) implementations.
- Replaced `position_map` implementation from `HashMap` to [`HashTable`](https://docs.rs/hashbrown/0.14.5/hashbrown/struct.HashTable.html) improving key lookup speeds.
- `search` and `range_query` support `count` and `sorted` arguments.

## 0.2.0

- Added support for OpenAPI schema generation by adding a `utoipa` crate feature, which conditionally derive `utoipa::ToSchema` trait.

## 0.1.0

> Initial release

This release introduces the core features for geospatial proximity detection in the Geoprox project.

### Key Features

- **Geohash Indexing**: Utilizes [`patricia_tree::map::StringPatriciaMap`](https://docs.rs/patricia_tree/0.8.0/patricia_tree/map/type.StringPatriciaMap.html) to index geohashes. This approach optimizes in-memory search space and efficiently locates keys with the same geohash prefix, allowing for rapid geographic proximity searches.
- **Localized Search**: Implements localized search functionality using [`kiddo::KdTree`](https://docs.rs/kiddo/4.2.0/kiddo/type.KdTree.html) for precise nearest neighbor calculations.
