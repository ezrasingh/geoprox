# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.1.0

> Initial release

This service provides an API wrapper over [geoprox-core](../geoprox-core/README.md).

### Notes

- HTTP REST service built on [tokio-rs/axum](https://github.com/tokio-rs/axum)
- OpenAPI documentation generation via [utoipa](https://github.com/juhaku/utoipa)
- Server supports `cache-control` response headers with compatible clients
