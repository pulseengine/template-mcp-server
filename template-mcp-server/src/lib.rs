//! Template MCP Server Library
//!
//! This template provides a starting point for building MCP servers using the
//! PulseEngine MCP framework. It demonstrates:
//! - Using the #[mcp_server] macro for automatic server setup
//! - Using the #[mcp_tools] macro for automatic tool discovery
//! - Basic tool implementations with different parameter types
//! - Proper error handling and async support

use pulseengine_mcp_macros::{mcp_server, mcp_tools};
use serde::{Deserialize, Serialize};

/// Example data structure that your tools might work with
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExampleData {
    pub id: u64,
    pub name: String,
    pub value: f64,
    pub tags: Vec<String>,
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
#[derive(Default, Clone)]
pub struct TemplateMcpServer {
    // Add your server state here
    // Example: 
    // data_store: Arc<RwLock<HashMap<u64, ExampleData>>>,
}

/// All public methods in this impl block become MCP tools automatically
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