# Chickchat 🐣

This app is my first fullstack Rust application. It's written in Leptos for the fronted (CSR) and Rocket for the backend.

## Frontend

To execute the frontend you can run:

```bash
trunk serve --open
```

For more about how to install `trunk`, follow the [official documentation](https://book.leptos.dev/getting_started/index.html#hello-world-getting-set-up-for-leptos-csr-development).

## Backend

You'll need to to have running a `MySQL` database server and install [diesel](https://diesel.rs/guides/getting-started#installing-diesel-cli), so you can run the migrations:

```bash
diesel migration run
```

Set the `env` file inside the backend (look at the `.env.example`). After that you can open the server:

```bash
cargo watch -x run
```