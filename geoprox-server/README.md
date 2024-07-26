# GeoProx Server API

## Overview

GeoProx Server provides a geospatial indexing API with capabilities to encode/decode geohashes, query for neighboring geohashes, and perform geospatial searches within a specified range. This API allows you to efficiently manage and query geospatial data.

## Table of Contents

1. [Geohash API](#geohash-api)
    - [Encode Coordinates into Geohash](#encode-coordinates-into-geohash)
    - [Decode Geohash into Coordinates](#decode-geohash-into-coordinates)
    - [Neighboring Regions](#neighboring-regions)
2. [Geoshard API](#geoshard-api)
    - [Search Nearby](#search-nearby)
    - [Create Geospatial Index](#create-geospatial-index)
    - [Insert Key into Index](#insert-key-into-index)
    - [Drop Index](#drop-index)
    - [Remove Key from Index](#remove-key-from-index)
3. [Components](#components)
    - [Schemas](#schemas)
5. [License](#license)

## API Endpoints

### Geohash API

#### Encode Coordinates into Geohash

- **Endpoint:** `/api/v1/geohash`
- **Method:** `GET`
- **Summary:** Encode coordinates by query params, returns geohash.
- **Parameters:**
  - `lat` (number, required): Latitude
  - `lng` (number, required): Longitude
  - `depth` (integer, required): Determines geohash length
- **Responses:**
  - `200`: Object with geohash encoded latitude/longitude
    - Example:
      ```json
      {
        "geohash": "u4pruydqqvj"
      }
      ```

#### Decode Geohash into Coordinates

- **Endpoint:** `/api/v1/geohash/{ghash}`
- **Method:** `GET`
- **Summary:** Decode geohash by path param, returns coordinates with precision estimates.
- **Parameters:**
  - `ghash` (string, required): Geohash encoded region
- **Responses:**
  - `200`: Object with latitude/longitude pair and precision errors
    - Example:
      ```json
      {
        "lat": 57.64911,
        "lat_error": 0.00001,
        "lng": 10.40744,
        "lng_error": 0.00001
      }
      ```

#### Neighboring Regions

- **Endpoint:** `/api/v1/geohash/{ghash}/neighbors`
- **Method:** `GET`
- **Summary:** Returns geohash neighbors in all cardinal directions.
- **Parameters:**
  - `ghash` (string, required): Geohash encoded region
- **Responses:**
  - `200`: Object with geohash neighbors
    - Example:
      ```json
      {
        "sw": "u4pruydqqv",
        "s": "u4pruydqvr",
        "se": "u4pruydqqw",
        "w": "u4pruydqqt",
        "e": "u4pruydqqx",
        "nw": "u4pruydqqm",
        "n": "u4pruydqqn",
        "ne": "u4pruydqql"
      }
      ```

### Geoshard API

#### Search Nearby

- **Endpoint:** `/api/v1/shard/{index}`
- **Method:** `GET`
- **Summary:** Search geospatial index for all keys within some distance.
- **Parameters:**
  - `lat` (number, required): Latitude
  - `lng` (number, required): Longitude
  - `range` (integer, required): Search radius in kilometers
  - `index` (string, required): Geospatial index name
- **Responses:**
  - `200`: List of resource keys found within range
    - Example:
      ```json
      {
        "found": [
          {"key": "resource1", "distance": 0.5},
          {"key": "resource2", "distance": 1.2}
        ]
      }
      ```

#### Create Geospatial Index

- **Endpoint:** `/api/v1/shard/{index}`
- **Method:** `POST`
- **Summary:** Creates an in-memory index within this geoshard.
- **Parameters:**
  - `index` (string, required): Geospatial index name
- **Responses:**
  - `201`: Index creation status
    - Example:
      ```json
      {
        "created": true,
        "existed": false
      }
      ```

#### Insert Key into Index

- **Endpoint:** `/api/v1/shard/{index}`
- **Method:** `PUT`
- **Summary:** Inserts key into geospatial index.
- **Parameters:**
  - `index` (string, required): Geospatial index name
- **Request Body:**
  - `key` (string, required): Resource key
  - `lat` (number, required): Latitude
  - `lng` (number, required): Longitude
- **Responses:**
  - `201`: Key insertion status
    - Example:
      ```json
      {
        "key": "resource1",
        "geohash": "u4pruydqqvj"
      }
      ```

#### Drop Index

- **Endpoint:** `/api/v1/shard/{index}`
- **Method:** `DELETE`
- **Summary:** Deletes geospatial index, all keys will be lost.
- **Parameters:**
  - `index` (string, required): Geospatial index name
- **Responses:**
  - `202`: Index deletion status
    - Example:
      ```json
      {
        "deleted": true
      }
      ```

#### Remove Key from Index

- **Endpoint:** `/api/v1/shard/{index}`
- **Method:** `PATCH`
- **Summary:** Remove key from geospatial index.
- **Parameters:**
  - `index` (string, required): Geospatial index name
- **Request Body:**
  - `key` (string, required): Resource key
- **Responses:**
  - `200`: Key removal status
    - Example:
      ```json
      {
        "key": "resource1",
        "deleted": true
      }
      ```

## Components

### Schemas

#### EncodeLatLng
- **Description:** Arguments for encoding latitude/longitude as geohash.
- **Properties:**
  - `lat` (number, required): Latitude
  - `lng` (number, required): Longitude
  - `depth` (integer, required): Determines geohash length (1-10)

#### EncodeLatLngResponse
- **Description:** Returns geohash encoded latitude/longitude.
- **Properties:**
  - `geohash` (string, required): Geohash string

#### DecodeGeohashResponse
- **Description:** Returns geohash decoded as latitude/longitude with precision errors.
- **Properties:**
  - `lat` (number, required): Latitude
  - `lat_error` (number, required): Latitude error
  - `lng` (number, required): Longitude
  - `lng_error` (number, required): Longitude error

#### GeohashNeighborsResponse
- **Description:** Neighboring geohash regions.
- **Properties:**
  - `sw` (string, required): Southwest neighbor
  - `s` (string, required): South neighbor
  - `se` (string, required): Southeast neighbor
  - `w` (string, required): West neighbor
  - `e` (string, required): East neighbor
  - `nw` (string, required): Northwest neighbor
  - `n` (string, required): North neighbor
  - `ne` (string, required): Northeast neighbor

#### CreateIndexResponse
- **Description:** Returns index creation status.
- **Properties:**
  - `created` (boolean, required): True if index was created
  - `existed` (boolean, required): True if index already existed

#### DropIndexResponse
- **Description:** Returns index deletion status.
- **Properties:**
  - `deleted` (boolean, required): True if index was deleted

#### InsertKey
- **Description:** Arguments for inserting a key.
- **Properties:**
  - `key` (string, required): Resource key
  - `lat` (number, required): Latitude
  - `lng` (number, required): Longitude

#### InsertKeyResponse
- **Description:** Returns key and geohash.
- **Properties:**
  - `key` (string, required): Resource key
  - `geohash` (string, required): Geohash encoded latitude/longitude

#### QueryRangeResponse
- **Description:** Returns resource keys found with their distance.
- **Properties:**
  - `found` (array, required): List of keys and their distances
    - `key` (string): Resource key
    - `distance` (number): Distance in kilometers

#### RemoveKey
- **Description:** Arguments for removing a key.
- **Properties:**
  - `key` (string, required): Resource key

#### RemoveKeyResponse
- **Description:** Returns key and deletion status.
- **Properties:**
  - `deleted` (boolean, required): True if key was removed
  - `key` (string, required): Resource key

## License

This project is licensed under the [Apache 2.0](../LICENSE-APACHE) or [MIT License](../LICENSE-MIT) (your choice).
