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

To get started run `just setup` and start a `geoprox-server` instance from the project root.
If you are using a port other than the default, update the `API_SPEC` variable in your `.env` file.

## Using Just

The `just` command runner is included to help manage and run tasks easily. Here are the available tasks:

```shell
Available recipes:
    build               # build all client libraries
    codegen TARGET_LANG # generate sdk client
    default             # default recipe to display help information
    manifest            # build generator configurations
    setup               # setup dotenv file
```

Here is an example usage building only the Rust client `just codegen rust`

## Configuring Generators

All generator configs are defined in the `manifest.jsonnet` file, this file is used to template and build configs in the `generators/` directory.

For information on available generators and configuration options, refer to the [OpenAPI Generator documentation](https://openapi-generator.tech/docs/generators/).

## Using Nix

If you prefer to use [Nix](https://nixos.org/) for managing dependencies, you can enter a development shell with the required environment by running:

```shell
nix-shell
```

This command will set up the environment with all the necessary dependencies configured. Once inside the Nix shell, you can proceed with running `just setup` and starting the `geoprox-server` instance as described above.
