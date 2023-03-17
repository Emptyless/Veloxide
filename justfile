# Show available commands
default:
  @just --list --justfile {{justfile()}}

# Perform linting. It is advised to lint in your IDE instead of running this command.
lint:
	@echo "It is advised to lint in your IDE instead of running this command"
	cargo clippy

# Run the application supporting containers, then run the binary
dev features="postgres":
	docker-compose up -d
	cargo run --features {{features}}

# Stop the containers in docker (this stops the docker stack)
stop:
	docker-compose down

# Restart the containers in docker (this restarts the docker stack)
restart: stop dev

# Generates a code coverage report to be viewed in your IDE.
cover:
	cargo llvm-cov report --lcov --output-path ./coverage/lcov.info

# Generate a HTML coverage report and open it
coverhtml:
	cargo llvm-cov --html
	open target/llvm-cov/html/index.html

# Install the required tools for development with Veloxide
install-required:
	@echo "Installing tools..."

	@echo "Installing cargo-llvm-cov (code coverage report generation: https://github.com/taiki-e/cargo-llvm-cov)"
	cargo install cargo-llvm-cov

	@echo "Installing sqlx-cli (database migrations: https://crates.io/crates/sqlx-cli)"
	cargo install sqlx-cli --no-default-features --features postgres,mysql,sqlite,rustls

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