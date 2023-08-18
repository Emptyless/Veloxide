set dotenv-load := true

# Show available commands
default:
    @just --list --justfile {{justfile()}}

# Run the application supporting containers, then run the binary
dev: fmt up migrate
    cargo run -p veloxide-server | bunyan

# Run the application supporting containers, then run the frontend with hot reloading
dev-frontend: 
    cd ./frontends/sveltekit && pnpm run dev

# Stop the containers in docker (this stops the docker stack)
stop:
    docker-compose down

# Restart the containers in docker (this restarts the docker stack)
restart: stop dev

# Generates a code coverage report to be viewed in your IDE.
cover: fmt
    cargo llvm-cov report --lcov --output-path ./coverage/lcov.info

# Generate a HTML coverage report and open it
coverhtml: fmt
    cargo llvm-cov --html
    open target/llvm-cov/html/index.html

# Install the required tools for development with Veloxide
install-required:
	@echo "Installing tools..."

	@echo "Installing cargo-llvm-cov (code coverage report generation: https://github.com/taiki-e/cargo-llvm-cov)"
	cargo install cargo-llvm-cov

	@echo "Installing sqlx-cli (database migrations: https://crates.io/crates/sqlx-cli)"
	cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite,rustls

	@echo "Installing ruplacer (replacement tool: https://github.com/your-tools/ruplacer)"
	cargo install ruplacer

	@echo "Installing bunyan (log parser tool: https://github.com/LukeMathWalker/bunyan)"
	cargo install bunyan

	@echo "Installing ripgrep (search tool: https://github.com/BurntSushi/ripgrep)"
	cargo install ripgrep

	@echo "Installing mdbook (book tool: https://github.com/rust-lang/mdBook)"
	cargo install mdbook && cargo install mdbook-toc

	@echo "Installing Rust nightly toolchain"
	rustup toolchain install nightly

	@echo "Installing tools...Done"

# Install recommended tooling that isn't required
install-recommended:
	@echo "Installing recommended tools..."

	@echo "Installing bacon (background code checker: https://github.com/Canop/bacon)"
	cargo install bacon

	@echo "Installing cargo-watch (hot reloading: https://crates.io/crates/cargo-watch)"
	cargo install cargo-watch

	@echo "Installing recommended tools... Done"

# Install both the required and recommended tools
install-all: install-required install-recommended

# Opens the user guide in your browser
guide:
    mdbook watch ./docs/guide --open

[private]
fmt-nightly:
    rustup default nightly
    cargo fmt --all
    rustup default stable

[private]
fmt:
    cargo fmt --all

[private]
up:
    docker-compose up -d

# Restarts the OPA container, useful when you've changed the policy
restart-opa:
    docker-compose stop opa
    docker-compose rm -f opa
    docker-compose up -d opa
    docker-compose logs -f opa

# Tests the policies defined in /policies
test-policies:
    opa test ./policies -v

# Creates a sqlx offline file for usage in the CI/CD pipeline
sqlx-prepare:
    cd ./backend/crates/veloxide-server && cargo sqlx prepare

# Generate the stubs for the frontend(s) from the protobuf definitions
gen:
    cd ./contracts && protoc --proto_path=. ./*.proto --ts_out=../frontends/sveltekit/src/lib/stubs --plugin=protoc-gen-ts=../frontends/sveltekit/node_modules/.bin/protoc-gen-ts

 
# Perform the database migrations
migrate:
    cd ./backend/crates/veloxide-server/ && cargo sqlx database create && cargo sqlx migrate run

# Deploy the backend to fly.io
deploy:
    act -j deploy -s FLY_API_TOKEN=$FLY_API_TOKEN

# Check unused dependencies
udeps:
    cargo +nightly udeps

