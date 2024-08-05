# default recipe to display help information
default:
  @just --list

# Runs cargo fmt (nightly)
fmt ARGS='':
  cargo +nightly fmt {{ARGS}}

# Run geoprox CLI
geoprox +ARGS='--help':
    cargo run -- {{ARGS}}

