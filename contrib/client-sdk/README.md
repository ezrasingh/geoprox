# Client SDK Generator

This is a development environment for generating client SDKs using [OpenAPI Generator](https://github.com/OpenAPITools/openapi-generator/). [Jsonnet](https://jsonnet.org/) is utilized to template OpenAPI Generator configurations, and [Just](https://github.com/casey/just/) is used to run the OpenAPI Generator CLI.

## Dependencies

The development environment requires the following dependencies:

- Node.js (npx)
- Yarn (@openapitools/openapi-generator-cli)
- JRE (Java Runtime Environment)
- Just
- jsonnet

Once these tools are installed, you can run `yarn install` to install the required Node.js packages.

## Getting Started

## Using Just

The `just` command runner is included to help manage and run tasks easily. Here are the available tasks:

```sh
Available recipes:
    build               # Build all client libraries
    codegen TARGET_LANG # Generate sdk client
    default             # Default recipe to display help information
    manifest            # Build generator configurations
    spec                # Generate OpenAPI spec
```

Here’s how to build only the Rust client using the OpenAPI specification:

```sh
just spec manifest codegen rust
```

This command generates the Rust client library from the OpenAPI specification. It will compile the spec and produce the client code without additional components.

## Configuring Generators

All generator configs are defined in the `manifest.jsonnet` file, this file is used to template and build configs in the `generators/` directory.

For information on available generators and configuration options, refer to the [OpenAPI Generator documentation](https://openapi-generator.tech/docs/generators/).

## Using Nix

If you prefer to use [Nix](https://nixos.org/) for managing dependencies, you can enter a development shell with the required environment by running:

```sh
nix-shell
```

This command will set up the environment with all the necessary dependencies configured. Once inside the Nix shell, you can proceed with running `just setup` and starting the `geoprox-server` instance as described above.

## Troubleshooting Rust Client Build Issues

### OpenSSL Development Headers Not Found

If you encounter the following error while building `geoprox-client`:

```sh
Failed to find OpenSSL development headers.

  You can try fixing this setting the `OPENSSL_DIR` environment variable
  pointing to your OpenSSL installation or installing OpenSSL headers package
  specific to your distribution:

      # On Ubuntu
      sudo apt-get install pkg-config libssl-dev
      # On Arch Linux
      sudo pacman -S pkgconf openssl
      # On Fedora
      sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel
      # On Alpine Linux
      apk add pkgconf openssl-dev
```

These commands install the necessary OpenSSL development headers for your distribution. Ensure you've installed OpenSSL correctly.

If the error persists after trying the suggested fixes, refer to the solution in [`rust-openssl#2217`](https://github.com/sfackler/rust-openssl/issues/2217#issuecomment-2230398481)
. This solution applied to my case as I was using Ubuntu 24, which required additional configuration. Following the instructions in the mentioned issue resolved the build problem for me.
