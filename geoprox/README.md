# Geoprox

> A standalone runtime for the [Geoprox project](https://github.com/ezrasingh/geoprox/).

[Geoprox](https://github.com/ezrasingh/geoprox/) is an in-memory geospatial search engine designed for efficient real-time location-based pairing.

It helps quickly identify users or entities near a specific location, making it ideal for applications that need to match contracts with nearby users. Whether it's for ride-sharing services like Uber and Lyft or delivery platforms like Grubhub, Geoprox provides the speed and accuracy needed to enhance geo-aware interactions.

[Discussed @ May 2024 Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024/-/blob/main/Geo-Proximity-Detection-With-Rust.pdf)

This crate offers a command-line tool for launching and managing the Geoprox server.

## Installation

To install `geoprox`, use the following command:

```sh
cargo install geoprox
```

## Usage

Geoprox provides several commands to start the server and work with geohashes:

```sh
geoprox help
Usage: geoprox [OPTIONS] [COMMAND]

Commands:
  run     Start Geoprox server
  encode  Hash latitude/longitude into geohash
  decode  Decode geohash into approximate longitude/latitude
  spec    Generate an OpenAPI specification file
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Specify a config file
  -h, --help             Print help
  -V, --version          Print version
```

## Getting Started

To start using Geoprox, you can run the following command to start the server:

```sh
geoprox run
```

You can also encode and decode geohashes directly from the CLI:

```sh
# Encode latitude and longitude into geohash
geoprox encode --lat=37.7749 --lng=-122.4194 --depth=5

# Decode geohash into latitude and longitude
geoprox decode 9q8yy
```

For detailed help on any command, use the help command:

```sh
geoprox help encode
```

## Configuration

Geoprox allows specifying a configuration file using the `-c` or `--config` option or set the `GEOPROX_CONFIG` environment variable. This file can contain various settings to customize the behavior of the Geoprox server and commands. The configuration can be provided in any common format such as `YAML`, `TOML`, `JSON`, or `INI`.

### Example Config

Here's an example configuration file in `TOML` format:

```toml
# All settings are optional; these are the default values.
[server]
# The address the server will bind to
http_addr = '0.0.0.0'
# The port the server will listen on
http_port = 5000
# Timeout duration in seconds
timeout = 10

[shard]
# Determines the default geohash length for inserts
insert_depth = 6
# Determines the default geohash length for searches
search_depth = 6
# Specifies the default number of results returned in range queries
default_count = 100
# Toggles the default sorting behavior for query results
default_sorted = false
```

#### Environment Variables

These are the currently supported environment variables. They will take precedence over settings defined in the configuration file.

| Environment Variable | Description                            | Default Value               |
| -------------------- | -------------------------------------- | --------------------------- |
| `GEOPROX_CONFIG`     | Specifies the configuration file path. | `/etc/geoprox/geoprox.toml` |
| `GEOPROX_HTTP_ADDR`  | The address the server will bind to.   | `0.0.0.0`                   |
| `GEOPROX_HTTP_PORT`  | The port the server will listen on.    | `5000`                      |

## Fine Tuning

Geohashes divide the world into a grid of cells, each with a unique identifier (encoded in base 32). The precision of the geohash can be adjusted by changing its length:

- Short geohashes represent larger areas.
- Long geohashes represent smaller, more precise areas.

| Geohash Length | Lat Bits | Lng Bits | Lat Error | Lng Error | Km Error                   |
| -------------- | -------- | -------- | --------- | --------- | -------------------------- |
| 1              | 2        | 3        | ±23       | ±23       | ±2,500 km (1,600 mi)       |
| 2              | 5        | 5        | ±2.8      | ±5.6      | ±630 km (390 mi)           |
| 3              | 7        | 8        | ±0.70     | ±0.70     | ±78 km (48 mi)             |
| 4              | 10       | 10       | ±0.087    | ±0.18     | ±20 km (12 mi)             |
| 5              | 12       | 13       | ±0.022    | ±0.022    | ±2.4 km (1.5 mi; 2,400 m)  |
| 6              | 15       | 15       | ±0.0027   | ±0.0055   | ±0.61 km (0.38 mi; 610 m)  |
| 7              | 17       | 18       | ±0.00068  | ±0.00068  | ±0.076 km (0.047 mi; 76 m) |
| 8              | 20       | 20       | ±0.000085 | ±0.00017  | ±0.019 km (0.012 mi; 19 m) |

For example, a **depth of 6 corresponds to a geohash precision of approximately 1km x 1km**. This is the default depth and generally recommended, as greater precision (smaller depths) may not be necessary for most use cases.

The `insert_depth` and `search_depth` settings control the precision of geohashing and impact the performance of distance calculations and search queries.

### Optimizing `insert_depth`

**Higher Insert Depth:**

- **Description:** Objects are grouped into smaller, more precise regions.
- **Impact:** Searching takes slightly longer, but distance calculations are more accurate.
- **Use Case:** Ideal for scenarios where precise distance measurements are crucial.

**Lower Insert Depth:**

- **Description:** Objects are grouped into larger regions.
- **Impact:** Search performance improves, but distance calculations become less accurate.
- **Use Case:** Best for cases where fast general distance estimates are acceptable.

### Optimizing `search_depth`

**Higher Search Depth:**

- **Description:** The initial search region is smaller and more precise.
- **Impact:** Search queries take slightly longer, but results are more accurate.
- **Use Case:** Suitable for narrow range queries where high accuracy is needed.

**Lower Search Depth:**

- **Description:** The initial search region is larger, leading to faster searches.
- **Impact:** Search speed improves, but accuracy may be reduced.
- **Use Case:** Ideal for wide range queries where speed is prioritized over precision.

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-APACHE) or [MIT License](https://github.com/ezrasingh/geoprox/tree/main/LICENSE-MIT) (your choice).
