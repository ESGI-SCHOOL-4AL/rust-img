# Rust-img-api

This project aims at making a web API with **Go** to expose a **Rust** library.

## Prerequisites

- Rust **nightly**
- Go 1.13+
- Go Modules enabled

## Run

```
make run
```

Press CTRL+C to shutdown the HTTP server.

### Routes

- `GET http://localhost:8080`: Index, with user interface for ease of use.
- `POST http://localhost:8080/invert`: Invert the given ppm image.
- `POST http://localhost:8080/grayscale`: Grayscale the given ppm image.

## Build

```
make build
```

The generated executable will be located in the root folder of the project and named `rust-img-api`.
