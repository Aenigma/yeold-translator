# Yeold

## Description
Translates text into the TF2 Medieval mode style text.

## Building

You can run the command line application using cargo run:

```bash
cargo run
```

Clone the repo, and then build the wasm module using wasm-pack.

```bash
wasm-pack build --target browser
```

Then, go to the `web` directory, install dependencies, and build the frontend:

```bash
cd web/
pnpm install
pnpm build
```

# Development

You may find it useful to run the tests in watch mode:

```bash
RUST_BACKTRACE=1 cargo watch -x "test -- --show-output"
```

And run the vite dev server with HMR:

```bash
cd web/
pnpm dev
```
