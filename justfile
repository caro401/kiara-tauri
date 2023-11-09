# List all the things you can do with this justfile, with descriptions
help:
  @just --list

# Run the project for local development
dev:
    @echo "don't forget to start kiara service - just start-kiara"
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
    cd src-tauri/ && cargo fmt
    npm run format

# Run static analysis on the code
lint:
    cd src-tauri/ && cargo clippy
    npm run lint
    npm run check

# Run a kiara server via pixi
start-kiara:
    pixi run backend

clean-kiara:
    @echo "go delete wherever kiara stores its application data, bad luck"
#    pixi run clear-context
