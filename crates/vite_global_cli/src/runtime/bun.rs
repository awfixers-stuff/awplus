//! Bun runtime implementation.
//!
//! This module provides Bun runtime management. Currently, bun is not fully
//! integrated - the shim simply passthrough to the system bun.
//!
//! TODO: Implement the full runtime trait when bun version management is ready.
//!
//! ## Version Sources
//!
//! Bun supports version resolution from:
//! - `.bun-version` file in project root
//! - `package.json` `engines.bun` field
//! - `bunfig.toml` `[install]` section
//! - `BUN_VERSION` environment variable
//! - Global default (from `bun config`)
//!
//! ## Installation
//!
//! Bun can be installed via:
//! - Direct download from api.bun.sh
//! - `bun upgrade` command
//! - System package managers

use vite_path::{AbsolutePath, AbsolutePathBuf};
use vite_shared::get_vite_plus_home;

use super::{Resolution, Runtime};
use crate::error::Error;

pub struct BunRuntime;

impl Runtime for BunRuntime {
    fn name(&self) -> &'static str {
        "bun"
    }

    fn shim_tools(&self) -> &[&str] {
        &["bun", "bunx"]
    }

    fn resolve_version(&self, cwd: &AbsolutePath) -> Result<Resolution, Error> {
        // TODO: Implement version resolution for Bun
        // This should read:
        // - .bun-version file
        // - package.json engines.bun
        // - bunfig.toml [install] version
        // - BUN_VERSION env var
        // - Global default
        Err(Error::Message(
            "Bun version resolution is not yet implemented. Use system bun.".to_string(),
        ))
    }

    fn ensure_installed(&self, version: &str) -> Result<(), Error> {
        // TODO: Implement bun installation
        // Downloads bun from api.bun.sh/install
        let home_dir = get_vite_plus_home()
            .map_err(|e| Error::Message(format!("Failed to get vite-plus home dir: {e}")))?
            .join("js_runtime")
            .join("bun")
            .join(version);

        let binary_path = home_dir.join("bin").join("bun");

        if binary_path.as_path().exists() {
            return Ok(());
        }

        Err(Error::Message(format!(
            "Bun {} is not installed. Please install bun manually or use system bun.",
            version
        )))
    }

    fn locate_tool(&self, version: &str, tool: &str) -> Result<AbsolutePathBuf, Error> {
        let home_dir = get_vite_plus_home()
            .map_err(|e| Error::Message(format!("Failed to get vite-plus home dir: {e}")))?
            .join("js_runtime")
            .join("bun")
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
        use vite_command::resolve_bin;
        use vite_path::current_dir;

        // Simple PATH lookup for system bun
        let path_var = std::env::var_os("PATH")?;
        let cwd = current_dir().ok()?;
        resolve_bin(tool, Some(&path_var), &cwd).ok()
    }
}
