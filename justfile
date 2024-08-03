# default recipe to display help information
default:
  @just --list

# Runs cargo fmt (nightly)
fmt ARGS='':
  cargo +nightly fmt {{ARGS}}

# Run geoprox CLI
geoprox ARGS='':
    cargo run -- {{ARGS}}

