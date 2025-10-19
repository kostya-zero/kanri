use anyhow::Result;
use std::{
    io::ErrorKind,
    process::{Command, Stdio},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("Failed to launch program: {0}")]
    ProgramNotFound(String),

    #[error("Process was interrupted")]
    ProcessInterrupted,

    #[error("No permission to execute the program")]
    NoPermission,

    #[error("Program exited with non-zero status: {0}")]
    NonZeroExitCode(i32),

    #[error("An unexpected error occurred: {0}")]
    UnexpectedError(String),
}

#[derive(Debug, Clone, Default)]
pub struct LaunchOptions {
    pub program: String,
    pub args: Vec<String>,
    pub cwd: Option<String>,
    pub fork_mode: bool,
    pub quiet: bool,
    pub env: Option<Vec<(String, String)>>,
}

pub fn launch_program(options: LaunchOptions) -> Result<(), ProgramError> {
    let mut cmd = Command::new(&options.program);

    if options.quiet {
        cmd.stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null());
    } else {
        cmd.stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());
    }

    cmd.args(options.args);
    if let Some(cwd_path) = options.cwd {
        cmd.current_dir(cwd_path);
    }

    if let Some(env) = options.env {
        cmd.envs(env);
    }

    if options.fork_mode {
        // In fork mode, we just spawn and don't wait for completion
        cmd.spawn().map_err(|e| match e.kind() {
            ErrorKind::NotFound => ProgramError::ProgramNotFound(options.program.to_string()),
            ErrorKind::PermissionDenied => ProgramError::NoPermission,
            ErrorKind::Interrupted => ProgramError::ProcessInterrupted,
            _ => ProgramError::UnexpectedError(e.to_string()),
        })?;
    } else {
        // Required only for Windows because if user runs program inside a shell and presses Ctrl+C,
        // user will lose control over the shell. I can't figure out why it is happening.
        // On Linux and macOS Ctrl+C works as expected.
        #[cfg(windows)]
        let _ = ctrlc::set_handler(|| {});

        let status = cmd.status().map_err(|e| match e.kind() {
            ErrorKind::NotFound => ProgramError::ProgramNotFound(options.program.to_string()),
            ErrorKind::PermissionDenied => ProgramError::NoPermission,
            ErrorKind::Interrupted => ProgramError::ProcessInterrupted,
            _ => ProgramError::UnexpectedError(e.to_string()),
        })?;

        if !status.success() {
            return if let Some(code) = status.code() {
                Err(ProgramError::NonZeroExitCode(code))
            } else {
                Err(ProgramError::ProcessInterrupted)
            };
        }
    }

    Ok(())
}
