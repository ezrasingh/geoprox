
default:
  @just --list

# run development server
run:
    cargo watch -x 'run -- run-server --bind 0.0.0.0:5000'