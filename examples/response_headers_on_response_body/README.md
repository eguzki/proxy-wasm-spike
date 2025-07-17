## Proxy-Wasm plugin example: send response headers at response body time

Proxy-Wasm plugin that performs sends response headers at response body phase

### Building

```sh
$ cargo build --target wasm32-wasip1 --release
```

### Using in Envoy

This example can be run with [`docker compose`](https://docs.docker.com/compose/install/)
and has a matching Envoy configuration.

```sh
$ docker compose up
```

#### Run

Send HTTP request to `localhost:10000/hello`:

```sh
$ curl localhost:10000/hello
```

#### Clean up

```sh
$ docker compose down
```
