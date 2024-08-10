<div align="center">
  <br>
  <h1>Geoprox 🌎</h1>
</div>
<br>
<p align="center">
  <a href="https://crates.io/search?q=geoprox">
    <img src="https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Crates">
  </a>
  <a href="https://github.com/ezrasingh/geoprox/tree/main/contrib/client-sdk">
    <img src="https://img.shields.io/badge/openapiinitiative-%23000000.svg?style=for-the-badge&logo=openapiinitiative&logoColor=white" alt="OpenAPI Spec">
  </a>
  <a href="https://hub.docker.com/repository/docker/ezrasingh/geoprox/">
    <img src="https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white" alt="Docker Hub">
  </a>
  <a href="https://github.com/ezrasingh/geoprox/actions/">
    <img src="https://img.shields.io/badge/github%20actions-%232671E5.svg?style=for-the-badge&logo=githubactions&logoColor=white" alt="GitHub Actions">
  </a>
  <a href="https://github.com/ezrasingh/geoprox/actions/workflows/ci.yml">
    <img src="https://github.com/ezrasingh/geoprox/actions/workflows/ci.yml/badge.svg" alt="Build Status">
  </a>
  <a href="https://github.com/ezrasingh/geoprox/actions/workflows/audit.yml">
    <img src="https://github.com/ezrasingh/geoprox/actions/workflows/audit.yml/badge.svg" alt="Audit Status">
  </a>
</p>

## What is Geoprox

Geoprox is an in-memory geospatial search engine designed for efficient real-time location-based pairing.

It helps quickly identify users or entities near a specific location, making it ideal for applications that need to match contracts with nearby users.

Whether it's for ride-sharing services like Uber and Lyft or delivery platforms like Grubhub, Geoprox provides the speed and accuracy needed to enhance geo-aware interactions.

[Discussed @ May 2024 Rust Indy.rs meetup](https://gitlab.com/indyrs/may2024/-/blob/main/Geo-Proximity-Detection-With-Rust.pdf)

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

- [geoprox](geoprox/) - Standalone CLI for running the Geoprox service
- [geoprox-core](geoprox-core/) - Core data structure implementations
- [geoprox-server](geoprox-server/) - HTTP API wrapper around `geoprox-core`
- [geoprox-client](contrib/client-sdk/rust/) - HTTP client library for `geoprox-server`

### API Client Libraries

HTTP Client libraries are generated from an OpenAPI specification (made available from the `/api-spec/openapi.json` endpoint) using [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator/).

Availble clients can be found at [`contrib/client-sdk`](contrib/client-sdk/), along with steps for generating your own client if needed.

## Getting Started

Use `cargo run` to access the Geoprox CLI. For additional information try `cargo run -- --help`.

### Using Just

The [`just` command runner](https://github.com/casey/just) is included to help manage and run tasks easily. Here are the available tasks:

```sh
Available recipes:
    default                # Default recipe to display help information
    fmt +ARGS=''           # Run cargo formatter
    geoprox +ARGS='--help' # Geoprox CLI
```

To start using Geoprox, you can run the following command to start the server:

```sh
just geoprox run
```

You can also encode and decode geohashes directly from the CLI:

```sh
# Encode latitude and longitude into geohash
just geoprox encode --lat=37.7749 --lng=-122.4194 --depth=5

# Decode geohash into latitude and longitude
just geoprox decode 9q8yy
```

For detailed help on any command, use the `help` command:

```sh
just geoprox help encode
```

### Using Docker

The Geoprox Docker image(`ezrasingh/geoprox`) is published to both [`docker.io`](https://hub.docker.com/repository/docker/ezrasingh/geoprox/) and [`ghcr.io`](https://github.com/ezrasingh/geoprox/pkgs/container/geoprox) registries.

For more details check out the [Docker Guide](contrib/docker/).

## Testing and Benchmarks

You can run tests and benchmarks using the following commands:

- **Run Tests:**

  ```sh
  cargo test
  ```

- **Run Benchmarks:**
  ```sh
  cargo bench
  ```

The project uses [Criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking. Benchmark results are saved in HTML format in the `target/criterion/` directory. You can view the latest benchmark reports online at [this link](https://ezrasingh.github.io/geoprox/bench/).

## Contributing

Contributions are welcome! Please see the [contributing guidelines](CONTRIBUTING.md) for more information.

## License

This project is licensed under the [Apache 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) (your choice).
