# Stargem dev commands

default:
  @just --list

# Build Docker image via Nix
docker-image:
  nix build ./server#dockerImage && skopeo copy --policy containers/policy.json docker-archive:result containers-storage:stargem-backend:latest

# Start services (default foreground). Add -d to detach.
up *flags="":
  podman-compose up {{flags}}

# Build Docker image and start services
up-docker: docker-image
  podman-compose up

# Start with dev profile (includes adminer), auto-build image
up-dev: docker-image
  podman-compose --profile dev up

# Stop services
down *flags="":
  podman-compose down {{flags}}

# Stop dev profile services
down-dev *flags="":
  podman-compose --profile dev down {{flags}}

# View logs
logs *flags="":
  podman-compose logs -f {{flags}}

# Regenerate proto stubs from protos/ and format them
proto:
  nix develop ./server -c bash -c "cd server && PROTO_SRC=../protos cargo build && cargo fmt"

# Check that committed proto stubs are up-to-date with protos/ and formatted
proto-check:
  nix develop ./server -c bash -c "cd server && PROTO_SRC=../protos cargo build && cargo fmt"
  git -C server diff --exit-code
