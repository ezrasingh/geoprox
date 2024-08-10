# Getting Started

Geoprox comes in different flavors depending on what you're cooking up. If you're looking to dive deep and use core features directly in your Rust projects, the `geoprox-core` crate is your go-to. It's packed with all the fundamental data structures that make Geoprox tick.

But if you're aiming for a more out-of-the-box experience, the main `geoprox` package is where the magic happens. It's the CLI runtime designed to sidecar with your other application services, making it super easy to integrate Geoprox into your existing setup.

## Crates

- [geoprox](https://github.com/ezrasingh/geoprox/tree/main/geoprox/) - Standalone CLI for running the Geoprox service
- [geoprox-core](https://github.com/ezrasingh/geoprox/tree/main/geoprox-core/) - Core data structure implementations
- [geoprox-server](https://github.com/ezrasingh/geoprox/tree/main/geoprox-server/) - HTTP API wrapper around `geoprox-core`
- [geoprox-client](https://github.com/ezrasingh/geoprox/tree/main/contrib/client-sdk/rust/) - HTTP client library for `geoprox-server`

## Installation

To install `geoprox`, use the following command:

```sh
cargo install geoprox
```

### Usage

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
