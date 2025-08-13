//! Template MCP Server Library
//!
//! This template provides a starting point for building MCP servers using the
//! PulseEngine MCP framework. It demonstrates:
//! - Using the #[mcp_server] macro for automatic server setup
//! - Using the #[mcp_tools] macro for automatic tool and resource discovery
//! - Basic tool implementations with different parameter types
//! - Resource implementations for read-only data access
//! - URI templates for parameterized resources
//! - Proper error handling and async support

use pulseengine_mcp_macros::{mcp_server, mcp_tools, mcp_resource};
use serde::{Deserialize, Serialize};

/// Example data structure that your tools might work with
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExampleData {
    pub id: u64,
    pub name: String,
    pub value: f64,
    pub tags: Vec<String>,
}

/// Server status information (exposed as a resource)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerStatus {
    pub name: String,
    pub version: String,
    pub uptime_seconds: u64,
    pub tools_count: usize,
    pub resources_count: usize,
}

/// Server configuration (exposed as a resource)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerConfig {
    pub max_concurrent_requests: usize,
    pub timeout_seconds: u64,
    pub debug_mode: bool,
    pub supported_formats: Vec<String>,
}

/// Template MCP Server
///
/// Replace this with your own server implementation. The #[mcp_server] macro
/// automatically generates the necessary MCP infrastructure.
#[mcp_server(
    name = "Template MCP Server",
    version = "0.1.0", 
    description = "A template MCP server demonstrating basic functionality",
    auth = "disabled"  // Change to "memory", "file", or remove for production
)]
#[derive(Clone)]
pub struct TemplateMcpServer {
    start_time: std::time::Instant,
    // Add your server state here
    // Example: 
    // data_store: Arc<RwLock<HashMap<u64, ExampleData>>>,
}

impl Default for TemplateMcpServer {
    fn default() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
}

/// All public methods in this impl block become MCP tools or resources automatically
/// Methods with #[mcp_resource] become resources, others become tools
#[mcp_tools]
impl TemplateMcpServer {
    /// Get server status and basic information
    /// 
    /// This is a simple tool that requires no parameters and returns
    /// a status message about the server.
    pub async fn get_status(&self) -> anyhow::Result<String> {
        Ok("Template MCP Server is running and ready to serve requests".to_string())
    }

    /// Echo back a message with optional prefix
    ///
    /// Demonstrates a tool with both required and optional parameters.
    /// 
    /// # Parameters
    /// - message: The message to echo back (required)
    /// - prefix: Optional prefix to add to the message
    pub async fn echo(&self, message: String, prefix: Option<String>) -> anyhow::Result<String> {
        match prefix {
            Some(p) => Ok(format!("{}: {}", p, message)),
            None => Ok(format!("Echo: {}", message)),
        }
    }

    /// Add two numbers together
    ///
    /// Demonstrates a tool that works with numeric parameters.
    /// 
    /// # Parameters
    /// - a: First number
    /// - b: Second number
    pub async fn add_numbers(&self, a: f64, b: f64) -> anyhow::Result<f64> {
        Ok(a + b)
    }

    /// Create example data
    ///
    /// Demonstrates a tool that creates and returns structured data.
    ///
    /// # Parameters  
    /// - name: Name for the data entry
    /// - value: Numeric value
    /// - tags: Optional list of tags
    pub async fn create_data(
        &self,
        name: String,
        value: f64,
        tags: Option<Vec<String>>,
    ) -> anyhow::Result<ExampleData> {
        Ok(ExampleData {
            id: rand::random::<u64>(),
            name,
            value,
            tags: tags.unwrap_or_default(),
        })
    }

    /// Process a list of items
    ///
    /// Demonstrates working with arrays/lists as parameters.
    ///
    /// # Parameters
    /// - items: List of strings to process
    /// - operation: Operation to perform ("count", "join", "reverse")
    pub async fn process_list(
        &self,
        items: Vec<String>,
        operation: String,
    ) -> anyhow::Result<String> {
        match operation.as_str() {
            "count" => Ok(format!("List contains {} items", items.len())),
            "join" => Ok(items.join(", ")),
            "reverse" => {
                let reversed: Vec<String> = items.into_iter().rev().collect();
                Ok(reversed.join(", "))
            }
            _ => Err(anyhow::anyhow!(
                "Unknown operation: {}. Supported: count, join, reverse",
                operation
            )),
        }
    }

    /// Example of a tool that might fail
    ///
    /// Demonstrates proper error handling in MCP tools.
    ///
    /// # Parameters
    /// - should_fail: If true, the tool will return an error
    pub async fn example_with_error(&self, should_fail: bool) -> anyhow::Result<String> {
        if should_fail {
            Err(anyhow::anyhow!("This tool was asked to fail"))
        } else {
            Ok("Tool executed successfully".to_string())
        }
    }

    // Resources - Read-only data accessible via MCP resource URIs
    
    /// Get server status information
    ///
    /// This resource provides read-only access to the server's current status.
    /// Resources are for data that clients need to read but not modify.
    #[mcp_resource(
        uri_template = "template://server-status",
        name = "server_status",
        description = "Current server status and statistics",
        mime_type = "application/json"
    )]
    pub async fn server_status_resource(&self) -> anyhow::Result<ServerStatus> {
        let uptime = self.start_time.elapsed();
        
        Ok(ServerStatus {
            name: "Template MCP Server".to_string(),
            version: "0.1.0".to_string(),
            uptime_seconds: uptime.as_secs(),
            tools_count: 6, // Update this if you add/remove tools
            resources_count: 3, // Update this if you add/remove resources
        })
    }

    /// Get server configuration
    ///
    /// This resource exposes the server's configuration settings.
    /// Resources are perfect for configuration data that clients need to read.
    #[mcp_resource(
        uri_template = "template://server-config",
        name = "server_config", 
        description = "Server configuration settings",
        mime_type = "application/json"
    )]
    pub async fn server_config_resource(&self) -> anyhow::Result<ServerConfig> {
        Ok(ServerConfig {
            max_concurrent_requests: 100,
            timeout_seconds: 30,
            debug_mode: cfg!(debug_assertions),
            supported_formats: vec![
                "json".to_string(),
                "text".to_string(),
                "binary".to_string(),
            ],
        })
    }

    /// Get example data by ID
    ///
    /// This resource demonstrates parameterized resources using URI templates.
    /// The {id} parameter is extracted from the URI when the resource is accessed.
    #[mcp_resource(
        uri_template = "template://example-data/{id}",
        name = "example_data",
        description = "Example data entry by ID", 
        mime_type = "application/json"
    )]
    pub async fn example_data_resource(&self, id: String) -> anyhow::Result<ExampleData> {
        // In a real implementation, you'd look up the data by ID
        // For this template, we'll generate example data
        let id_num = id.parse::<u64>().unwrap_or(1);
        
        Ok(ExampleData {
            id: id_num,
            name: format!("Example Item {}", id_num),
            value: (id_num as f64) * 1.5,
            tags: vec![
                "example".to_string(),
                "template".to_string(),
                format!("id-{}", id_num),
            ],
        })
    }
}

// Add any additional implementation methods here that are NOT tools
// (private methods, helper functions, etc.)
impl TemplateMcpServer {
    // Example private helper method
    #[allow(dead_code)]
    fn internal_helper(&self) -> String {
        "This method is not exposed as an MCP tool".to_string()
    }
}