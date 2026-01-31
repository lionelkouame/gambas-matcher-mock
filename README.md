# Gambas Matcher Mock

**Gambas Matcher Mock** is an API interception and simulation utility written in Rust. Designed to be completely transparent to your applications (PHP or others), it acts as a smart proxy capable of bypassing calls to unstable APIs (banks, third-party services) to return local mocks based on specific rules.

## Features

- 🎯 **Smart Request Matching**: Match requests based on method, path, regex patterns, headers, and query parameters
- 🔄 **Flexible Mocking**: Return mock responses with custom status codes, headers, and body content
- 📁 **File-based Responses**: Load mock data from external files for complex responses
- ⚡ **High Performance**: Built with Rust and Tokio for asynchronous, non-blocking operation
- 🔧 **Easy Configuration**: Simple YAML-based configuration for defining rules and mocks
- 🌐 **Transparent Proxy**: Works seamlessly with any application without code changes

## Use Cases

- **Development**: Mock unstable or rate-limited APIs during development
- **Testing**: Create predictable test environments with controlled API responses
- **Offline Development**: Work without internet or external API dependencies
- **API Simulation**: Simulate third-party services (payment gateways, banking APIs, etc.)
- **Integration Testing**: Test edge cases and error scenarios

## Installation

### Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)

### Building from Source

```bash
git clone https://github.com/lionelkouame/gambas-matcher-mock.git
cd gambas-matcher-mock
cargo build --release
```

The compiled binary will be available at `target/release/gambas-matcher-mock`.

### Using Docker

You can also run Gambas Matcher Mock using Docker:

```bash
# Build the Docker image
docker build -t gambas-matcher-mock .

# Run with example configuration
docker run -p 8080:8080 -v $(pwd)/examples/config.yaml:/app/config.yaml gambas-matcher-mock

# Or use Docker Compose
docker-compose up
```

## Quick Start

1. **Create a configuration file** (`config.yaml`):

```yaml
server:
  host: "127.0.0.1"
  port: 8080

rules:
  - name: "Get User"
    matcher:
      method: "GET"
      path: "/api/users/123"
    response:
      status: 200
      headers:
        Content-Type: "application/json"
      body: |
        {
          "id": 123,
          "name": "John Doe"
        }
```

2. **Run the server**:

```bash
./target/release/gambas-matcher-mock --config config.yaml
```

3. **Test the mock**:

```bash
curl http://127.0.0.1:8080/api/users/123
```

## Configuration

### Server Configuration

```yaml
server:
  host: "127.0.0.1"  # Host to bind to
  port: 8080         # Port to listen on
```

### Defining Rules

Each rule consists of a **matcher** and a **response**:

```yaml
rules:
  - name: "Rule Name"
    matcher:
      # Matching criteria
    response:
      # Mock response
```

### Matcher Options

#### Basic Matching

```yaml
matcher:
  method: "GET"           # HTTP method (GET, POST, PUT, DELETE, etc.)
  path: "/api/users/123"  # Exact path match
```

#### Regex Path Matching

```yaml
matcher:
  method: "GET"
  path_regex: "^/api/users/[0-9]+$"  # Match paths with regex
```

#### Header Matching

```yaml
matcher:
  method: "POST"
  path: "/api/data"
  headers:
    Authorization: "Bearer token123"
    Content-Type: "application/json"
```

#### Query Parameter Matching

```yaml
matcher:
  method: "GET"
  path: "/api/products"
  query_params:
    category: "electronics"
    sort: "price"
```

### Response Options

#### Inline Response Body

```yaml
response:
  status: 200
  headers:
    Content-Type: "application/json"
  body: |
    {
      "message": "Success"
    }
```

#### File-based Response Body

```yaml
response:
  status: 200
  headers:
    Content-Type: "application/json"
  body_file: "mocks/user_data.json"
```

## Usage Examples

### Example 1: Mock a Banking API

```yaml
- name: "Bank Balance"
  matcher:
    method: "GET"
    path: "/bank/api/v1/balance"
    headers:
      Authorization: "Bearer token123"
  response:
    status: 200
    headers:
      Content-Type: "application/json"
    body: |
      {
        "balance": 15750.50,
        "currency": "USD"
      }
```

### Example 2: Simulate Error Responses

```yaml
- name: "API Error"
  matcher:
    method: "POST"
    path: "/api/payment"
  response:
    status: 500
    headers:
      Content-Type: "application/json"
    body: |
      {
        "error": "Payment processing failed"
      }
```

### Example 3: Dynamic Path Matching

```yaml
- name: "Any User ID"
  matcher:
    method: "GET"
    path_regex: "^/api/users/[0-9]+$"
  response:
    status: 200
    headers:
      Content-Type: "application/json"
    body: |
      {
        "id": 999,
        "name": "Mock User"
      }
```

## Command Line Options

```bash
gambas-matcher-mock [OPTIONS]

Options:
  -c, --config <FILE>    Path to configuration file [default: config.yaml]
      --host <HOST>      Override host from config
  -p, --port <PORT>      Override port from config
  -v, --verbose          Enable verbose logging
  -h, --help             Print help
  -V, --version          Print version
```

## Integration with PHP Applications

To use Gambas Matcher Mock with PHP applications:

1. **Configure your application** to use the mock server URL:

```php
// Instead of:
$apiUrl = 'https://api.bank.com/v1/balance';

// Use:
$apiUrl = 'http://127.0.0.1:8080/bank/api/v1/balance';
```

2. **Or use environment variables**:

```php
$apiUrl = getenv('API_BASE_URL') ?: 'http://127.0.0.1:8080';
```

3. **No other code changes required!** Your application will transparently use the mock API.

## Logging

The server logs all incoming requests and matching results:

```
[INFO] Starting Gambas Matcher Mock server on 127.0.0.1:8080
[INFO] Loaded 5 rules
[INFO] Received request: GET /api/users/123
[INFO] Matched rule: Get User
[INFO] Returning mock response with status 200
```

Use the `-v` flag for verbose logging:

```bash
gambas-matcher-mock --config config.yaml -v
```

## Examples

See the `examples/` directory for more configuration examples:

- `examples/config.yaml` - Complete configuration with various matching patterns
- `examples/mocks/` - Sample mock response files

## Development

### Running Tests

```bash
cargo test
```

### Running in Development Mode

```bash
cargo run -- --config examples/config.yaml
```

### Building

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Lionel Kouame

## Acknowledgments

Built with:
- [Tokio](https://tokio.rs/) - Asynchronous runtime
- [Hyper](https://hyper.rs/) - HTTP library
- [Serde](https://serde.rs/) - Serialization framework
