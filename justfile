
default:
  @just --list

# run development redis
docker:
    docker compose up

# run development server
run:
    cargo watch -x run

frontend:
    yarn --cwd frontend dev