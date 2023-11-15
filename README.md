# Prototype kiara mini-app for network analysis

## developing

This is built with [tauri](https://tauri.app) and [sveltekit](https://kit.svelte.dev/) and [tailwindcss](https://tailwindcss.com/)

### for now

Set up a python environment for pyo3 to use. This needs python-devel or install via pyenv with `env PYTHON_CONFIGURE_OPTS="--enable-shared" pyenv install 3.11.6`

Instruct PyO3 to use the correct Python version with an env variable: `export PYO3_PYTHON=/Users/XXX/.pyenv/versions/3.11.6/bin/python` or whatever for your OS

### always

- have a [node](https://nodejs.org/en) environment and rust installed (via [rustup](https://rustup.rs/))
- `npm install`
- `npm run tauri dev`

## building

- `npm run tauri build`
