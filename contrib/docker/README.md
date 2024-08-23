# Docker Guide

The Geoprox Docker image (`ezrasingh/geoprox`) is available on two registries:

- [Docker Hub](https://hub.docker.com/repository/docker/ezrasingh/geoprox/)
- [GitHub Container Registry](https://github.com/ezrasingh/geoprox/pkgs/container/geoprox)

## Getting Started

> To quickly get started with Docker Compose, skip to the [Quick Start Guide](#quick-start).

Pull the Geoprox container from one of the available repositories.

```sh
# from Docker Hub
docker pull ezrasingh/geoprox:latest

# or from GitHub Container Registry
docker pull ghcr.io/ezrasingh/geoprox:latest
```

To run the container with default configurations, use:

```sh
docker run -t -p 5000:5000 ezrasingh/geoprox:latest
```

This command will start the Geoprox container with the default settings, exposing port 5000 for access.

> To use another port on your machine try `-p <my-port>:5000`

### Variants & Tagging

The Geoprox image supports two variant tags: `alpine` and `debian`.

By default, the `latest` tag uses the `alpine` variant. Latest is built from the `main` branch.

To pin your Geoprox instance to a specific version and variant, use the format `<version>-<variant>`, such as `v0.3.0-debian`.

> If you specify only the version, like `v0.3.0`, it will default to the `alpine` variant, resulting in `v0.3.0-alpine`.

Check the respective registry for available tags.

### Customizing the Configuration

> For complete settings and configuration details, refer to the [Geoprox CLI README](../../geoprox/README.md#configuration).

Geoprox allows specifying a configuration file using the `-c` or `--config` option or set the `GEOPROX_CONFIG` environment variable. Geoprox will automatically detect and parse configuration files in formats like `YAML`, `TOML`, `JSON`, or `INI` if they are named `geoprox.yaml`, `geoprox.toml`, `geoprox.json`, or `geoprox.ini`, respectively.

For example, if you have a configuration file named `geoprox.toml` in the container working directory (`/var/lib/geoprox`), you can run the container with the following command:

```sh
docker run -t -p 5000:5000 \
    -v $(pwd)/geoprox.toml:/var/lib/geoprox/geoprox.toml:ro \
    ezrasingh/geoprox:latest \
    -c geoprox.toml
```

If you need to specify a different path or file name for your configuration, use the `-c` or `--config` option:

```sh
docker run -t -p 5000:5000 \
    -v $(pwd)/custom-config.yaml:/some/path/custom-config.yaml:ro \
    ezrasingh/geoprox:latest \
    -c /some/path/custom-config.yaml
```

In this command:

- `-v $(pwd)/custom-config.yaml:/some/path/custom-config.yaml:ro` mounts the local configuration file into the container.
- `-c /some/path/custom-config.yaml` specifies the configuration file to be used by the Geoprox server.

Or use the `GEOPROX_CONFIG` environment variable:

```sh
docker run -t -p 5000:5000 \
    -v $(pwd)/custom-config.yaml:/some/path/custom-config.yaml:ro \
    -e GEOPROX_CONFIG='/some/path/custom-config.yaml' \
    ezrasingh/geoprox:latest
```

In this command:

- `-e GEOPROX_CONFIG='/some/path/custom-config.yaml'` specifies the configuration file to be used by the Geoprox server.

## Quick Start

To quickly get started with Geoprox using Docker, follow these steps to configure and run the server with Docker Compose.

> The provided Docker Compose configuration serves as a boilerplate to showcase how Geoprox can be configured and allows you to get an instance up and running fast for evaluation purposes.

### Docker Compose

Use the following command to start the Geoprox service:

```sh
docker-compose -f quickstart/docker-compose.yml up
```

This command will start the Geoprox service using the configuration specified in [`quickstart/docker-compose.yml`](quickstart/docker-compose.yml). The [configuration file](quickstart/geoprox.toml) in this setup is a basic example, allowing you to easily evaluate Geoprox with default settings.

For custom configurations, you can modify the Docker Compose file or replace it with your own configuration as needed.
