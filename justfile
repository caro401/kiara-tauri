# List all the things you can do with this justfile, with descriptions
help:
  @just --list

# Run the project for local development
dev:
   env PYO3_PYTHON=$HOME/.kiara-app/python/bin/python npm run tauri dev

alias setup := install
# Install all project dependencies
install:
    npm i

# Update all project dependencies
update-deps:
    npm up

# Build the project for production deployment
build:
    env PYO3_PYTHON=$HOME/.kiara-app/python/bin/python npm run tauri build

alias fmt := format
# Run the code formatter
format:
    cd src-tauri/ && cargo fmt
    npm run format

# Run static analysis on the code
lint:
    cd src-tauri/ && cargo clippy
    npm run lint
    npm run check
