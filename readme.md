### Help command

```shell
cargo run -- --help

Run a local proxy to capture HTTP/HTTPS requests and expose them via API

Usage: intercept [OPTIONS]

Options:
      --proxy-port <PROXY_PORT>  Port to run the proxy server on (default: 8080) [default: 8080]
      --api-port <API_PORT>      Port to run the API server on (default: 3000) [default: 3000]
  -h, --help                     Print help
  -V, --version                  Print version
```

### Usage

Run the proxy with custom ports:

```bash
cargo run -- --proxy-port 9090 --api-port 4001
```
