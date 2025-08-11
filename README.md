# Template MCP Server

A template repository for creating Model Context Protocol (MCP) servers using the PulseEngine MCP framework in Rust.

## 🚀 Quick Start

1. **Use this template** by clicking the "Use this template" button on GitHub
2. **Clone your new repository**:
   ```bash
   git clone https://github.com/yourusername/your-mcp-server.git
   cd your-mcp-server
   ```
3. **Customize the server**:
   - Update `Cargo.toml` with your project details
   - Modify `src/lib.rs` to implement your tools
   - Update this README with your project information

4. **Build and test**:
   ```bash
   cargo build
   echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/template-mcp-server
   ```

## 🛠 What's Included

This template provides:

- **Complete MCP server setup** using PulseEngine MCP framework
- **Automatic tool discovery** with `#[mcp_tools]` macro
- **Example tools** demonstrating different parameter types:
  - Simple status check (no parameters)
  - Echo with optional parameters
  - Numeric calculations
  - Structured data creation
  - List processing
  - Error handling examples
- **STDIO transport** for integration with MCP clients
- **Proper logging configuration** for debugging

## 📁 Project Structure

```
template-mcp-server/
├── Cargo.toml                    # Workspace configuration
├── template-mcp-server/
│   ├── Cargo.toml                # Package configuration  
│   ├── src/
│   │   ├── main.rs               # Server entry point
│   │   └── lib.rs                # Server implementation & tools
├── README.md                     # This file
├── LICENSE                       # MIT License
└── .github/                      # GitHub templates
    ├── ISSUE_TEMPLATE/
    ├── PULL_REQUEST_TEMPLATE.md
    └── dependabot.yml
```

## 📦 Installation

### From npm (Recommended)

Install globally to use with any MCP client:

```bash
npm install -g @yourusername/template-mcp-server
```

Or use directly with npx:

```bash
npx @yourusername/template-mcp-server
```

### From Source

1. **Prerequisites**
   - Rust 1.75.0 or later
   - Git
   - Node.js 16+ (for npm distribution)

2. **Clone and Build**
   ```bash
   git clone https://github.com/yourusername/template-mcp-server.git
   cd template-mcp-server
   cargo build --release
   ```

3. **Run the Server**
   ```bash
   ./target/release/template-mcp-server
   ```

### Platform-Specific Binaries

Pre-built binaries are available for:
- macOS (x64, arm64)
- Linux (x64, arm64)
- Windows (x64)

Download from [GitHub Releases](https://github.com/yourusername/template-mcp-server/releases)

## 🔧 Development

### Building
```bash
cargo build
```

### Running
```bash
cargo run
```

### Testing with MCP Inspector
```bash
# Install MCP Inspector
npm install -g @modelcontextprotocol/inspector

# Test your server
npx @modelcontextprotocol/inspector ./target/debug/template-mcp-server
```

### Testing with Direct JSON-RPC
```bash
# List available tools
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}' | ./target/debug/template-mcp-server

# Call a tool
echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"get_status","arguments":{}}}' | ./target/debug/template-mcp-server
```

## 📝 Customizing Your Server

### 1. Update Package Information
Edit `template-mcp-server/Cargo.toml`:
```toml
[package]
name = "your-mcp-server"
description = "Your server description"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/your-mcp-server"
```

### 2. Implement Your Tools
In `src/lib.rs`, modify the `#[mcp_tools]` impl block:
```rust
#[mcp_tools]
impl YourMcpServer {
    /// Your custom tool
    pub async fn your_tool(&self, param: String) -> anyhow::Result<String> {
        // Your implementation here
        Ok(format!("Result: {}", param))
    }
}
```

### 3. Add Server State
Add fields to your server struct:
```rust
#[mcp_server(name = "Your Server")]
#[derive(Clone)]
pub struct YourMcpServer {
    data: Arc<RwLock<HashMap<String, String>>>,
    config: YourConfig,
}
```

### 4. Update Server Configuration
Modify the `#[mcp_server]` attributes:
```rust
#[mcp_server(
    name = "Your Amazing MCP Server",
    version = "1.0.0",
    description = "Does amazing things",
    auth = "file"  // or "memory", "disabled"
)]
```

## 🔌 Integration with MCP Clients

### Claude Desktop

Using npm installation:
```json
{
  "servers": {
    "your-server": {
      "command": "npx",
      "args": ["@yourusername/template-mcp-server"]
    }
  }
}
```

Using local binary:
```json
{
  "servers": {
    "your-server": {
      "command": "/path/to/your-mcp-server",
      "args": []
    }
  }
}
```

### Continue.dev

Using npm installation:
```json
{
  "mcpServers": {
    "your-server": {
      "command": "npx",
      "args": ["@yourusername/template-mcp-server"]
    }
  }
}
```

Using local binary:
```json
{
  "mcpServers": {
    "your-server": {
      "command": "/path/to/your-mcp-server"
    }
  }
}
```

## 📚 Framework Features

This template uses the PulseEngine MCP framework which provides:

- **Automatic tool discovery** - Public methods become MCP tools
- **Type-safe parameter handling** - Automatic JSON deserialization
- **Error handling** - Proper MCP error responses
- **Authentication** - Optional auth with multiple backends
- **Transport support** - STDIO, HTTP, WebSocket
- **Monitoring** - Built-in metrics and tracing
- **Validation** - Request/response validation

## 🔐 Authentication

The template includes authentication support:

- `auth = "disabled"` - No authentication (development)
- `auth = "memory"` - In-memory auth (testing)  
- `auth = "file"` - File-based auth (production)

For production use, configure file-based auth:
```rust
#[mcp_server(auth = "file")]
```

## 📊 Monitoring & Debugging

The server includes comprehensive logging. Set log levels:
```bash
RUST_LOG=debug ./target/debug/template-mcp-server
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes  
4. Add tests if applicable
5. Submit a pull request

## 📄 License

This template is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## 🆘 Support

- [PulseEngine MCP Documentation](https://docs.rs/pulseengine-mcp-protocol)
- [MCP Specification](https://modelcontextprotocol.io/specification/2025-06-18)
- [GitHub Issues](https://github.com/yourusername/your-mcp-server/issues)

## 🏷 Template Usage

When using this template:

1. **Click "Use this template"** on GitHub
2. **Create your repository** with a descriptive name
3. **Clone and customize** as described above
4. **Delete this section** from your README
5. **Update all placeholder information** with your project details

Happy building! 🎉