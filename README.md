# Anynet

A Rust crate for inspecting local network interfaces and reporting active server ports at startup.

## Overview

`anynet` is a lightweight Rust crate designed to help developers inspect local network interfaces and report active server ports when a server starts. It provides a simple macro, `anynet!`, to log network information such as localhost, local network IPs, and optionally the public IP address. The crate is ideal for command-line utilities, network programming, and web development projects.

## Features

- **Local Interface Detection**: Automatically detects and lists all non-loopback network interfaces.
- **Port Validation**: Ensures the provided port number is valid (between 1 and 65535).
- **Public IP Support**: Optionally retrieves and displays the public IP address.
- **Sorted IP Display**: Prioritizes common private IP ranges (e.g., 192.168.x.x, 10.x.x.x) for cleaner output.
- **Customizable Logging**: Integrates with the `fancy-log` crate for configurable log levels (Info, Debug, Warn, Error).
- **Simple Macro API**: Easy-to-use `anynet!` macro for quick integration.

## Installation

Add `anynet` to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
anynet = "1"
```

## Usage

The `anynet!` macro can be used to log network information for a given port. It supports two modes:

1. **Basic Usage** (logs localhost and local network IPs):
```rust
use anynet::anynet;

fn main() {
    anynet!(port = 8080);
}
```

2. **Public IP Mode** (includes public IP address):
```rust
use anynet::anynet;

fn main() {
    anynet!(port = 8080, public = true);
}
```

### Example Output

Running the demo with `LogLevel::Debug`:

```
--- Running demo with public=false (default) ---
✓ Listening on http://localhost:8080
✓ Listening on http://192.168.1.100:8080
✓ Listening on http://10.0.0.2:8080 +1 more
➜ Listening on http://172.16.0.3:8080

--- Running demo with public=true ---
✓ Listening on http://localhost:3000
✓ Listening on http://192.168.1.100:3000
✓ Listening on http://10.0.0.2:3000 +1 more
• Possible Public Network: http://203.0.113.1:3000
➜ Listening on http://172.16.0.3:3000

--- Running demo with invalid port ---
✗ Invalid port number: 0. Port must be between 1 and 65535.
```

## Example Code

The crate includes a demo example located in `examples/demo.rs`. To run it:

```bash
cargo run --example demo
```

This example demonstrates both modes of the `anynet!` macro and different log levels.

## Dependencies

- `fancy-log = "0.1"`: For styled logging output.
- `get_if_addrs = "0.5"`: For retrieving local network interfaces.
- `ip-lookup = "0.1"`: For fetching public IP addresses.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on the [GitHub repository](https://github.com/canmi21/anynet).

## Contact

For questions or feedback, please open an issue on the [GitHub repository](https://github.com/canmi21/anynet).