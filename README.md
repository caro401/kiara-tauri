# Prototype kiara mini-app for network analysis

## developing

This is built with [tauri](https://tauri.app) and [sveltekit](https://kit.svelte.dev/) and [tailwindcss](https://tailwindcss.com/)

- have a [node](https://nodejs.org/en) environment and rust installed (via [rustup](https://rustup.rs/))
- `npm install`
- `npm run tauri dev`

- for now, it assumes you have a kiara service running on `localhost:8080`. You can do this from this repo by having [pixi](https://github.com/prefix-dev/pixi) installed and running `pixi run backend`. At some point this should be bundled, but it isn't for now

## building

- `npm run tauri build`
