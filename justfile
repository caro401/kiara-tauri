# List all the things you can do with this justfile, with descriptions
help:
  @just --list

# Run the project for local development
dev:
    npm run tauri dev

alias setup := install
# Install all project dependencies
install:
    npm i

# Update all project dependencies
update-deps:
    npm up

# Build the project for production deployment
build:
    npm run tauri build

alias fmt := format
# Run the code formatter
format:
    npm run format
    cd src-tauri/ && cargo fmt

# Run static analysis on the code
lint:
    npm run lint
    npm run check
    cd src-tauri/ && cargo clippy

# Run a kiara server via pixi
start-kiara:
    pixi run backend