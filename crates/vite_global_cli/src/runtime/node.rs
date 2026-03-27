use vite_path::{AbsolutePath, AbsolutePathBuf};
use vite_shared::get_vite_plus_home;

use crate::error::Error;
use super::{Resolution, Runtime};

pub struct NodeRuntime;

impl Runtime for NodeRuntime {
    fn name(&self) -> &'static str {
        "node"
    }

    fn shim_tools(&self) -> &[&str] {
        &["node", "npm", "npx"]
    }

    fn resolve_version(&self, cwd: &AbsolutePath) -> Result<Resolution, Error> {
        // This is a simplified version - the full implementation uses caching
        // See dispatch.rs::resolve_with_cache for the complete logic
        crate::commands::env::config::resolve_version(cwd)
    }

    fn ensure_installed(&self, version: &str) -> Result<(), Error> {
        let home_dir = get_vite_plus_home()
            .map_err(|e| Error::Message(format!("Failed to get vite-plus home dir: {e}")))?
            .join("js_runtime")
            .join("node")
            .join(version);

        let binary_path = home_dir.join("bin").join("node");

        // Check if already installed
        if binary_path.as_path().exists() {
            return Ok(());
        }

        // Download the runtime
        crate::tokio::runtime()
            .block_on(async {
                vite_js_runtime::download_runtime(vite_js_runtime::JsRuntimeType::Node, version)
                    .await
            })
            .map_err(|e| Error::Message(format!("Failed to install Node.js: {e}")))?;

        Ok(())
    }

    fn locate_tool(&self, version: &str, tool: &str) -> Result<AbsolutePathBuf, Error> {
        let home_dir = get_vite_plus_home()
            .map_err(|e| Error::Message(format!("Failed to get vite-plus home dir: {e}")))?
            .join("js_runtime")
            .join("node")
            .join(version);

        let tool_path = home_dir.join("bin").join(tool);

        if !tool_path.as_path().exists() {
            return Err(Error::Message(format!(
                "Tool '{}' not found at {}",
                tool,
                tool_path.as_path().display()
            )));
        }

        Ok(tool_path)
    }

    fn find_system_tool(&self, tool: &str) -> Option<AbsolutePathBuf> {
        use crate::commands::env::config;
        use vite_command::resolve_bin;
        use vite_path::current_dir;
        use vite_shared::env_vars;

        let bin_dir = config::get_bin_dir().ok();
        let path_var = std::env::var_os("PATH")?;

        // Parse VITE_PLUS_BYPASS as a PATH-style list of additional directories to skip.
        let bypass_paths: Vec<std::path::PathBuf> = std::env::var_os(env_vars::VITE_PLUS_BYPASS)
            .map(|v| std::env::split_paths(&v).collect())
            .unwrap_or_default();

        // Filter PATH to exclude our bin directory and any bypass directories
        let filtered_paths: Vec<_> = std::env::split_paths(&path_var)
            .filter(|p| {
                if let Some(ref bin) = bin_dir {
                    if p == bin.as_path() {
                        return false;
                    }
                }
                !bypass_paths.iter().any(|bp| p == bp)
            })
            .collect();

        let filtered_path = std::env::join_paths(filtered_paths).ok()?;

        // Use vite_command::resolve_bin with filtered PATH - stops at first match
        let cwd = current_dir().ok()?;
        resolve_bin(tool, Some(&filtered_path), &cwd).ok()
    }
}
