## Git Commit format
[Sprint 1][Task 1] fix: add correct Docker Hub username to compose.yml 

#### Git Commit message setup
https://www.conventionalcommits.org/en/v1.0.0/

## Github Workflows
https://docs.github.com/en/actions/concepts/workflows-and-actions/workflows

## Docker Compose Docs
https://docs.docker.com/compose/

## Setup & Building
```bash
cargo install cargo-watch
cd app-service
cargo build
cd ..
cd auth-service
cargo build
cd ..
```

## Run servers locally (Manually)
#### App service
```bash
cd app-service
cargo watch -q -c -w src/ -w assets/ -w templates/ -x run
```

visit http://localhost:8000

#### Auth service
```bash
cd auth-service
cargo watch -q -c -w src/ -w assets/ -x run
```

visit http://localhost:3000

## Run servers locally (Docker)
```bash
docker compose build
docker compose up
```

visit http://localhost:8000 and http://localhost:3000