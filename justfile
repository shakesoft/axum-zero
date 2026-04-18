# justfile - Project unified command entry
set shell := ["pwsh", "-c"]
# check
check:
    cargo check &
    cd .. && cd antd-admin && pnpm lint

# Development mode: start backend + web together
dev:
    just dev-backend &
    just dev-web

# Start Rust backend (with hot reload)
dev-backend:
    cargo watch -x run -w src

# Start web (Vite dev mode)
dev-web:
    cd ./ && cd antd-admin && pnpm dev

# Build all (production)
build:
    just build-backend
    just build-web

# Build Rust backend release
build-backend:
    cargo build --release && copy ./config.toml ./target/release/config.toml

# Build web production bundle
build-web:
    cd ./ && cd antd-admin && pnpm build

# Clean build outputs
clean:
    rm -rf /target ../antd-admin/dist
