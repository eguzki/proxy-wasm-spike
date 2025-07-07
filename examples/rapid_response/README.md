## Proxy-Wasm plugin example: Return rapid response on http request headers

Proxy-Wasm plugin that will return rapid response on HTTP request headers.

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
