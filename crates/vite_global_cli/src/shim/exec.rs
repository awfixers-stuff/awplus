//! Platform-specific execution for shim operations.
//!
//! Uses execve to replace the current process on Unix.

use vite_path::AbsolutePath;
use vite_shared::output;

/// Convert a process ExitStatus to an exit code.
/// If the process was killed by a signal, returns 128 + signal_number.
fn exit_code_from_status(status: std::process::ExitStatus) -> i32 {
    use std::os::unix::process::ExitStatusExt;
    if let Some(signal) = status.signal() {
        return 128 + signal;
    }
    status.code().unwrap_or(1)
}

/// Spawn a tool as a child process and wait for completion.
///
/// Unlike `exec_tool()`, this does NOT replace the current process,
/// allowing the caller to run code after the tool exits.
pub fn spawn_tool(path: &AbsolutePath, args: &[String]) -> i32 {
    match std::process::Command::new(path.as_path()).args(args).status() {
        Ok(status) => exit_code_from_status(status),
        Err(e) => {
            output::error(&format!("Failed to execute {}: {}", path.as_path().display(), e));
            1
        }
    }
}

/// Execute a tool, replacing the current process.
pub fn exec_tool(path: &AbsolutePath, args: &[String]) -> i32 {
    use std::os::unix::process::CommandExt;

    let mut cmd = std::process::Command::new(path.as_path());
    cmd.args(args);

    // exec replaces the current process - this only returns on error
    let err = cmd.exec();
    output::error(&format!("Failed to exec {}: {}", path.as_path().display(), err));
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exit_code_from_status_normal() {
        let status =
            std::process::Command::new("/bin/sh").arg("-c").arg("exit 42").status().unwrap();
        assert_eq!(exit_code_from_status(status), 42);
    }

    #[test]
    fn test_exit_code_from_status_signal() {
        // Process kills itself with SIGINT (signal 2), expected exit code: 128 + 2 = 130
        let status =
            std::process::Command::new("/bin/sh").arg("-c").arg("kill -INT $$").status().unwrap();
        assert_eq!(exit_code_from_status(status), 130);
    }
}
