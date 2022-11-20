use std::{process::{Command, Stdio}, ffi::OsStr};

pub trait ShellCommand {
    /// command type
    type Cmd: AsRef<OsStr>;

    /// Executes the command as a child process,
    /// waiting for it to finish and collecting all of its output.
    fn exec_for_output<T>(cmd: &Self::Cmd, args: &[T])
    -> anyhow::Result<String> where T: AsRef<OsStr> {
        let child = Command::new(cmd)
            .args(args).stdout(Stdio::piped()).spawn()?;
        let output = child.wait_with_output()?;

        String::from_utf8(output.stdout)
            .or(Err(anyhow::anyhow!("Output the result failed!")))
    }

    /// Executes a command as a child process,
    /// waiting for it to finish and collecting its status.
    fn exec_for_status<T>(cmd: &Self::Cmd, args: &[T])
    -> anyhow::Result<i32> where T: AsRef<OsStr> {
        let mut child = Command::new(cmd)
            .args(args).stdout(Stdio::piped()).spawn()?;
        let status = child.wait()?;
        if let Some(code) = status.code() {
            Ok(code)
        } else {
            Err(anyhow::anyhow!("Process terminated by signal!"))
        }
    }
}

/// shell command: pwd
#[cfg(target_os = "linux")]
pub struct CmdPwd(String);

impl CmdPwd {
    #[cfg(target_os = "linux")]
    pub fn new() -> Self {
        CmdPwd("pwd".to_string())
    }

    pub fn cmd(&self) -> &String {
        &self.0
    }
}

impl ShellCommand for CmdPwd {
    type Cmd = String;
}

// #[cfg(target_os = "windows")]