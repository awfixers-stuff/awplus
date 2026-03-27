//! Runtime management abstraction for different JavaScript runtimes.
//!
//! This module provides a trait-based abstraction for managing different JavaScript
//! runtimes (Node.js, Bun, etc.) through a unified interface. This allows vite-plus
//! to support multiple runtimes while keeping the shim dispatch logic simple.
//!
//! ## Runtime Trait
//!
//! The [`Runtime`] trait defines the interface that all runtime implementations must support.
//! Each runtime (Node.js, Bun) implements this trait to provide:
//! - Version resolution from project files
//! - Runtime installation
//! - Tool binary location
//!
//! ## Current Implementations
//!
//! - [`NodeRuntime`] - Node.js runtime management (currently active)
//! - [`BunRuntime`] - Bun runtime management (coming soon)
//!
//! ## Adding a New Runtime
//!
//! To add support for a new runtime (e.g., Deno, QuickJS):
//!
//! 1. Create a new module `runtime/deno.rs`
//! 2. Implement the `Runtime` trait for your runtime
//! 3. Register the runtime in the `RUNTIMES` map
//! 4. Add shim tool detection in `shim/mod.rs`

mod node;
mod bun;

pub use node::NodeRuntime;
pub use bun::BunRuntime;

use vite_path::{AbsolutePath, AbsolutePathBuf};
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Resolution {
    pub version: String,
    pub source: String,
    pub project_root: Option<AbsolutePathBuf>,
    pub source_path: Option<AbsolutePathBuf>,
    pub is_range: bool,
}

pub trait Runtime: Send + Sync {
    /// The name of the runtime (e.g., "node", "bun")
    fn name(&self) -> &'static str;

    /// The shim tools this runtime manages (e.g., ["node", "npm", "npx"] for Node)
    fn shim_tools(&self) -> &[&str];

    /// Resolve the runtime version for a given working directory.
    ///
    /// This reads version files (.node-version, .bun-version), package.json, etc.
    fn resolve_version(&self, cwd: &AbsolutePath) -> Result<Resolution, Error>;

    /// Ensure the runtime is installed.
    ///
    /// Downloads and installs the runtime if not already present.
    fn ensure_installed(&self, version: &str) -> Result<(), Error>;

    /// Locate a tool binary within the runtime installation.
    ///
    /// For Node.js: finds node, npm, npx in the versioned bin directory
    /// For Bun: finds bun, bunx in the versioned bin directory
    fn locate_tool(&self, version: &str, tool: &str) -> Result<AbsolutePathBuf, Error>;

    /// Find the system-installed runtime in PATH.
    fn find_system_tool(&self, tool: &str) -> Option<AbsolutePathBuf>;
}

/// Get the runtime for a given tool name.
///
/// Returns the appropriate runtime implementation based on which shim tool
/// was invoked. For example, "node" → NodeRuntime, "bun" → BunRuntime.
pub fn get_runtime_for_tool(tool: &str) -> Option<&'static dyn Runtime> {
    // Node.js runtime
    if matches!(tool, "node" | "npm" | "npx") {
        return Some(&NodeRuntime as &'static dyn Runtime);
    }

    // Bun runtime - not yet implemented, returns None
    // TODO: Enable when BunRuntime is ready
    // if matches!(tool, "bun" | "bunx") {
    //     return Some(&BunRuntime as &'static dyn Runtime);
    // }

    None
}

/// Get runtime by name (for configuration).
pub fn get_runtime_by_name(name: &str) -> Option<&'static dyn Runtime> {
    match name {
        "node" => Some(&NodeRuntime as &'static dyn Runtime),
        // "bun" => Some(&BunRuntime as &'static dyn Runtime),
        _ => None,
    }
}
