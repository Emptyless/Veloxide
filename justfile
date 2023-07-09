set dotenv-load := true

# Show available commands
default:
  @just --list --justfile {{justfile()}}

# Run the application supporting containers, then run the binary
dev: fmt
	docker-compose up -d
	cargo prisma db push
	cargo run -p veloxide-server | bunyan

# Set the configuration to use postgres, then run the application supporting containers, then run the binary
dev-postgres: set-postgres dev

# Set the configuration to use mysql, then run the application supporting containers, then run the binary
dev-mysql: set-mysql dev

[private]
set-db db:
	ruplacer 'default = \["tracing", "graphql", "frontend", "(postgres|mysql)", "openapi"]' 'default = ["tracing", "graphql", "frontend", "{{db}}", "openapi"]' crates/veloxide-server/Cargo.toml --go
	ruplacer 'provider(.*?)= "(postgres|mysql)"' 'provider$1= "{{db}}"' prisma/schema.prisma --go
	ruplacer '(postgres|mysql)' {{db}} bacon.toml --go
	@echo "Default database in Cargo.toml set to {{db}}"
	@echo "Prisma database set to {{db}}, please ensure your DATABASE_URL is correct"

# Set the database to mysql
set-mysql: (set-db "mysql")
	ruplacer '^DATABASE_URL=.*' DATABASE_URL=$MYSQL_DATABASE_URL .env --go
	@echo "DATABASE_URL set to MYSQL_DATABASE_URL"
	cargo prisma generate

# Set the database to postgres
set-postgres: (set-db "postgres")
	ruplacer '^DATABASE_URL=.*' DATABASE_URL=$POSTGRES_DATABASE_URL .env --go
	@echo "DATABASE_URL set to POSTGRES_DATABASE_URL"
	cargo prisma generate

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
