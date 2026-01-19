# Makefile for cross-platform builds

.PHONY: help build build-all clean test run docker-build docker-run package

# Default target
help:
	@echo "Available targets:"
	@echo "  make build          - Build for current platform"
	@echo "  make build-all      - Build for all platforms"
	@echo "  make clean          - Clean build artifacts"
	@echo "  make test           - Run tests"
	@echo "  make run            - Run the server"
	@echo "  make docker-build   - Build Docker image"
	@echo "  make docker-run     - Run Docker container"
	@echo "  make package        - Create distribution packages"

# Detect OS
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	TARGET ?= x86_64-unknown-linux-gnu
	EXE_EXT :=
endif
ifeq ($(UNAME_S),Darwin)
	UNAME_P := $(shell uname -p)
	ifeq ($(UNAME_P),arm)
		TARGET ?= aarch64-apple-darwin
	else
		TARGET ?= x86_64-apple-darwin
	endif
	EXE_EXT :=
endif
ifeq ($(OS),Windows_NT)
	TARGET ?= x86_64-pc-windows-msvc
	EXE_EXT := .exe
endif

build:
	@echo "Building for $(TARGET)..."
	cd rust_samples && cargo build --release --target $(TARGET)

build-all:
	@echo "Building for all platforms..."
	@if command -v bash >/dev/null 2>&1; then \
		bash scripts/build-all.sh; \
	elif command -v pwsh >/dev/null 2>&1; then \
		pwsh scripts/build-all.ps1; \
	else \
		echo "Please run scripts/build-all.sh or scripts/build-all.ps1 manually"; \
	fi

clean:
	cd rust_samples && cargo clean

test:
	cd rust_samples && cargo test

run:
	cd rust_samples && cargo run

docker-build:
	docker build -t restor-octo-giggle:latest -f docker/Dockerfile.server .

docker-run:
	docker-compose -f docker/docker-compose.yml up

package:
	@if command -v bash >/dev/null 2>&1; then \
		bash scripts/package.sh; \
	else \
		echo "Please run scripts/package.sh manually"; \
	fi

# Platform-specific shortcuts
build-linux:
	cd rust_samples && cargo build --release --target x86_64-unknown-linux-gnu

build-windows:
	cd rust_samples && cargo build --release --target x86_64-pc-windows-msvc

build-macos:
	cd rust_samples && cargo build --release --target x86_64-apple-darwin
	cd rust_samples && cargo build --release --target aarch64-apple-darwin
