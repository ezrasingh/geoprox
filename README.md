# GeoProx

A Geo-Proximity detection service for efficient real-time geo-aware contract pairing.

This is a service that allows you to determine which users are nearby a contract and can be applied to applications like Uber, Lyft, Grubhub.

[Discussed @ Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024)

## Features

- **Geohash Encoding and Decoding**: Convert latitude and longitude coordinates to geohashes and vice versa.
- **Geohash Neighborhood**: Retrieve neighboring geohashes for a given geohash.
- **Geospatial Index Management**: Create, update, and delete geospatial indexes to manage geospatial data.
- **Proximity Searches**: Perform searches within a specified range to find nearby resources based on geohashes.

## Usage

### Geohash Operations
- **Encode Coordinates**: Convert latitude and longitude into a geohash for easy geospatial indexing.
- **Decode Geohash**: Retrieve latitude and longitude from a given geohash.
- **Get Neighbors**: Find neighboring geohashes in all cardinal directions from a given geohash.

### Geoshard Operations
- **Create Index**: Set up a new geospatial index to manage your data.
- **Insert Key**: Add a new key to an existing geospatial index.
- **Search Nearby**: Find keys within a specified range of coordinates.
- **Remove Key**: Delete a key from a geospatial index.
- **Drop Index**: Remove an entire geospatial index and its contents.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
