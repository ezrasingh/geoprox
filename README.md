# Geoprox

A Geo-Proximity detection service for efficient real-time geo-aware contract pairing.

This is a service that allows you to determine which users are nearby a contract and can be applied to applications like Uber, Lyft, Grubhub.

[Discussed @ Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024/-/blob/main/Geo-Proximity-Detection-With-Rust.pdf)

## Table of Contents

- [Table of Contents](#table-of-contents)
  - [Features](#high-level-features)
    - [Crates](#crates)
    - [Client Libraries](#api-client-libraries)
  - [Getting Started](#getting-started)
    - [Just](#using-just)
    - [Docker](#using-docker)
  - [Testing and Benchmarks](#testing-and-benchmarks)
  - [Contributing](#contributing)
  - [License](#license)

## High Level Features

- **Geohash Encoding and Decoding**: Convert latitude and longitude coordinates to geohashes and vice versa.
- **Geohash Neighborhood**: Retrieve neighboring geohashes for a given geohash.
- **Geospatial Index Management**: Create, update, and delete geospatial indexes to manage geospatial data.
- **Proximity Searches**: Perform searches within a specified range to find nearby resources based on geohashes.

### Crates

- [geoprox](geoprox/README.md) - Standalone CLI for running the Geoprox service
- [geoprox-core](geoprox-core/README.md) - Core data structure implementations
- [geoprox-server](geoprox-server/README.md) - HTTP API wrapper around `geoprox-core`
- [geoprox-client](contrib/client-sdk/rust/README.md) - HTTP client library for `geoprox-server`

### API Client Libraries

HTTP Client libraries are generated from an [OpenAPI](https://www.openapis.org/) spec (made available from the `/api-spec/openapi.json` endpoint) using [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator/).

Availble clients can be found at [`contrib/client-sdk`](contrib/client-sdk/README.md), along with steps for generating your own client if needed.

## Getting Started

Use `cargo run` to access the Geoprox CLI. For additional information try `cargo run -- --help`.

### Using Just

The [`just` command runner](https://github.com/casey/just) is included to help manage and run tasks easily. Here are the available tasks:

```shell
Available recipes:
    default                # default recipe to display help information
    fmt +ARGS=''           # Run cargo formatter
    geoprox +ARGS='--help' # Geoprox CLI
```

To start using Geoprox, you can run the following command to start the server:

```shell
just geoprox run
```

You can also encode and decode geohashes directly from the CLI:

```shell
# Encode latitude and longitude into geohash
just geoprox encode --lat=37.7749 --lng=-122.4194 --depth=5

# Decode geohash into latitude and longitude
just geoprox decode 9q8yy
```

For detailed help on any command, use the `help` command:

```shell
just geoprox help encode
```

### Using Docker

The Geoprox Docker image(`ezrasingh/geoprox`) is published to both [`docker.io`](https://hub.docker.com/repository/docker/ezrasingh/geoprox/) and [`ghcr.io`](https://github.com/ezrasingh/geoprox/pkgs/container/geoprox) registries.

For more details check out the [Docker Guide](contrib/docker/README.md).

## Testing and Benchmarks

Yu can run tests and benchmarks using the following commands:

- **Run Tests:**

  ```shell
  cargo test
  ```

- **Run Benchmarks:**
  ```shell
  cargo bench
  ```

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking. Benchmark results are saved in HTML format in the `target/criterion/` directory. You can view the latest benchmark reports online at [this link](https://ezrasingh.github.io/geoprox/bench/).

## Contributing

Contributions are welcome! Please see the [contributing guidelines](https://github.com/ezrasingh/geoprox/blob/main/CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
