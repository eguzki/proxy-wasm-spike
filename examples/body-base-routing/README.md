## Proxy-Wasm plugin example: gRPC request on HTTP request body and HTTP response body

Proxy-Wasm plugin that performs gRPC requests on HTTP request body phase and HTTP response body phase

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
curl -v http://127.0.0.1:10000/mcp \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "initialize",
    "params": {"protocolVersion":"2024-11-05","capabilities":{"sampling":{},"roots":{"listChanged":true}},"clientInfo":{"name":"my-client","version":"0.7.2"}}
  }'
```

```sh
curl -v http://127.0.0.1:10000/mcp \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json, text/event-stream' \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "tools/call",
    "params": {"name":"get_weather","arguments":{"location": "New York"}}
  }'
```

#### Clean up

```sh
$ docker compose down
```
