# Geoprox

A standalone runtime for the [Geoprox project](https://github.com/ezrasingh/geoprox/).

This crate offers a command-line tool for launching and managing the Geoprox server.

## Installation

To install `geoprox`, use the following command:

```shell
cargo install geoprox
```

## Usage

Geoprox provides several commands to start the server and work with geohashes:

```shell
geoprox help
Usage: geoprox [OPTIONS] [COMMAND]

Commands:
  run     Start Geoprox server
  encode  Hash latitude/longitude into geohash
  decode  Decode geohash into approximate longitude/latitude
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Specify a config file
  -h, --help             Print help
  -V, --version          Print version
```

## Getting Started

To start using Geoprox, you can run the following command to start the server:

```shell
geoprox run
```

You can also encode and decode geohashes directly from the CLI:

```shell
# Encode latitude and longitude into geohash
geoprox encode --lat=37.7749 --lng=-122.4194 --depth=5

# Decode geohash into latitude and longitude
geoprox decode 9q8yy
```

For detailed help on any command, use the help command:

```shell
geoprox help encode
```

## Configuration

Geoprox allows specifying a configuration file using the `-c` or `--config` option. This file can contain various settings to customize the behavior of the Geoprox server and commands. The configuration can be provided in any common format such as `YAML`, `TOML`, `JSON`, or `INI`.

### Example Configuration

Here's an example configuration file in `TOML` format:

```toml
[server]
http_addr = '0.0.0.0'
http_port = 5000

[shard]
insert_depth = 6
search_depth = 6
```

### Configuration Details

| Setting              | Description                                         |
| -------------------- | --------------------------------------------------- |
| `server.http_addr`   | The address the server will bind to.                |
| `server.http_port`   | The port the server will listen on.                 |
| `shard.insert_depth` | Determines the default geohash length for inserts.  |
| `shard.search_depth` | Determines the default geohash length for searches. |

The `insert_depth` and `search_depth` settings control the precision of the geohash. A longer geohash means more granular precision.

For example, a **depth of 6 corresponds to a geohash precision of approximately 1km x 1km**. This is the default depth and generally recommended, as greater precision (smaller depths) may not be necessary for most use cases.

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
