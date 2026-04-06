BINARY = dirhamly

.DEFAULT_GOAL := help
run:
	cargo run

dev:
	cargo check

build:
	cargo build

release:
	cargo build --release

fmt:
	cargo fmt

fmt-check:
	cargo fmt -- --check

clippy:
	cargo clippy --all-targets --all-features

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test

clean:
	cargo clean

ci: fmt-check lint test
	@echo "All checks passed ✅"

export:
	rm report.pdf
	cargo r -- export pdf
help:
	@echo ""
	@echo "Available commands:"
	@echo "  make run        - Run the project"
	@echo "  make dev        - cargo check"
	@echo "  make build      - Debug build"
	@echo "  make release    - Release build"
	@echo "  make fmt        - Format code"
	@echo "  make fmt-check  - Check formatting"
	@echo "  make clippy     - Run clippy"
	@echo "  make lint       - Clippy with warnings as errors"
	@echo "  make test       - Run tests"
	@echo "  make ci         - Run full CI checks"
	@echo "  make clean      - Clean target directory"
	@echo "  make export     - export data to pdf"
	@echo ""
