# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.5.0

- Added support for TTL and key-expiration ([#15](https://github.com/ezrasingh/geoprox/issues/15)).
- Added support for snapshots and persistence ([#17](https://github.com/ezrasingh/geoprox/issues/17)).
- Using [`duration-string`](https://crates.io/crates/duration-string) crate to handle `Duration` configuration.
- Bumped `geoprox-core` to [v0.5.0](https://crates.io/crates/geoprox-core/0.5.0).

# 0.4.2

- Added support for `Accept-Encoding`(via [`compression-full`](https://docs.rs/crate/tower-http/0.5.2/features#compression-full) feature in `tower-http`) and `Content-Encoding`(via [`decompression-full`](https://docs.rs/crate/tower-http/0.5.2/features#decompression-full) feature in `tower-http`) headers, [timeouts](https://docs.rs/crate/tower-http/0.5.2/features#timeout) and [tracing](https://docs.rs/crate/tower-http/0.5.2/features#trace) in responses.
- Added support for path normalization (i.e trailing slash or no trailing slash) ([#4](https://github.com/ezrasingh/geoprox/issues/4)).
- Bumped `geoprox-core` to [v0.4.2](https://crates.io/crates/geoprox-core/0.4.2).

## 0.4.1

- Added `server_config` and `shard_config` to `AppState` ([#2](https://github.com/ezrasingh/geoprox/issues/2))
- `api::routes` now takes in a single `AppState` as argument for building routes.
- Crate `README` now points to OpenAPI generated markdown docs
- Bumped `geoprox-core` to [v0.4.1](https://crates.io/crates/geoprox-core/0.4.1).

## 0.4.0

- Added missing OpenAPI `Path` parameter descriptions.
- Improved error handling and responses.
- Added `api::doc::openapi_spec_json()` for accesing the OpenAPI spec programmatically ([#3](https://github.com/ezrasingh/geoprox/issues/3)).
- Implemented Batch API for insertion, removal and query ([#1](https://github.com/ezrasingh/geoprox/issues/1)).
- Bumped `geoprox-core` to [v0.4.0](https://crates.io/crates/geoprox-core/0.4.0).

## 0.3.1

- Improved handling of edge cases involving the `count` parameter.
- Bumped `geoprox-core` to [v0.3.1](https://crates.io/crates/geoprox-core/0.3.1).

## 0.3.0

- `GET /api/v1/shard/:index` now supports optional `count` and `sorted` query parameters.
- Bumped `geoprox-core` to [v0.3.0](https://crates.io/crates/geoprox-core/0.3.0).

## 0.2.0

- Removed `KeysFound` enum which was a workaround to avoid packaging `geoprox-core` with [`utoipa`](https://crates.io/crates/utoipa) (since core shouldnt be concerned with OpenAPI). However, this was resolved by adding a crate feature on `geoprox-core` (see [v0.2.0](https://crates.io/crates/geoprox-core/0.2.0)) to conditionally derive `ToSchema` trait needed by `utoipa` for OpenAPI schema generation.
- Moved `ServerConfig` from `geoprox` to `geoprox-server` crate for future-proofing and improved seperation of concerns.
- Improved API error handling by returning structured errors in the response body, this was achieved using [`anyhow`](https://crates.io/crates/anyhow).

## 0.1.0

> Initial release

This service provides an API wrapper over [geoprox-core](../geoprox-core/README.md).

### Notes

- HTTP REST service built on [tokio-rs/axum](https://github.com/tokio-rs/axum).
- OpenAPI documentation generation via [utoipa](https://github.com/juhaku/utoipa).
- Server supports `cache-control` response headers with compatible clients.
