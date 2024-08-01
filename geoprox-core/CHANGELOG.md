# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## 0.1.0

> Initial release

This release introduces the core features for geospatial proximity detection in the Geoprox project.

### Key Features

- **Geohash Indexing**: Utilizes [`patricia_tree::map::StringPatriciaMap`](https://docs.rs/patricia_tree/0.8.0/patricia_tree/map/type.StringPatriciaMap.html) to index geohashes. This approach optimizes in-memory search space and efficiently locates keys with the same geohash prefix, allowing for rapid geographic proximity searches.
- **Localized Search**: Implements localized search functionality using [`kiddo::KdTree`](https://docs.rs/kiddo/4.2.0/kiddo/type.KdTree.html) for precise nearest neighbor calculations.