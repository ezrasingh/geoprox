# default recipe to display help information
default:
  @just --list

# Run cargo formatter
fmt +ARGS='':
  cargo +nightly fmt {{ARGS}}

# Geoprox CLI
geoprox +ARGS='--help':
    cargo run -- {{ARGS}}

