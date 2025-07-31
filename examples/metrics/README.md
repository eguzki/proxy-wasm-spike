## Proxy-Wasm plugin example: Prometheus metrics

Proxy-Wasm plugin that will emit prometheus metrics

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

Send HTTP request to `localhost:9080/hello`:

```sh
$ curl localhost:9080/hello
```

#### Read metrics

Send HTTP request to `localhost:9090/metrics`:

```sh
$ curl localhost:9090/stats/prometheus
```

#### Clean up

```sh
$ docker compose down
```
