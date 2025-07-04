<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos CSR with Actix Web API Server

This project demonstrates how to use Leptos in Client-Side Rendering (CSR) mode with a separate Actix Web API server.

## Project Structure

- **Leptos CSR Application**: A client-side rendered Leptos application
- **Actix Web API Server**: A separate API server that provides HTTP endpoints

## Running the Application

### 1. Start the API Server

```bash
cargo run --bin api_server --features api-server
```

The API server will start on http://127.0.0.1:3001.

### 2. Start the Leptos Application

```bash
trunk serve
```

The Leptos application will start on http://127.0.0.1:3000.

## Features

### Home View

The home view demonstrates how to fetch data from both:

1. Leptos server functions (which run in the browser in CSR mode)
2. The Actix Web API server (via HTTP requests)

It displays both responses side by side and includes a refresh button to update the data.

### API Endpoints

- `GET /api/hello`: Returns a simple hello message

## Adding New API Endpoints

To add new endpoints to the API server, edit the `src/api_server.rs` file. See the API_README.md file for more details.

## Project Setup

This project was created using the Leptos CSR template. If you want to create a similar project from scratch, follow these steps:

```sh
cargo install cargo-generate trunk leptosfmt
cargo generate --git https://github.com/leptos-community/start-csr
cd leptos_start
```

By default, this template uses Rust `nightly` and requires that you've installed the `wasm` compilation target for your toolchain.

If you don't have Rust nightly, you can install it with:
```sh
rustup toolchain install nightly --allow-downgrade
```

You can add the `wasm` compilation target to rust using:
```sh
rustup target add wasm32-unknown-unknown
```


## Development Notes

- In CSR mode, Leptos server functions don't actually create server endpoints; they run directly in the browser
- The API server provides actual HTTP endpoints that can be accessed directly
- This approach gives you flexibility during development and testing

## Developing your Leptos CSR project

To develop your Leptos CSR project, run:

```sh
# Start the API server in one terminal
cargo run --bin api_server --features api-server

# Start the Leptos application in another terminal
trunk serve --port 3000 --open
```

This will open your app in your default browser at `http://localhost:3000`.

## Deploying your Leptos CSR project

To build a Leptos CSR app for release, use the command:

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

For the API server, you would need to deploy it separately as a backend service.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].


[Leptos]: https://github.com/leptos-rs/leptos

[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/

[deploy-csr]: https://book.leptos.dev/deployment/csr.html