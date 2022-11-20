use crate::cmd::ShellCommand;

pub struct GitStatus(String, String);

impl GitStatus {
    pub fn new() -> Self {
        GitStatus("git".to_string(), "status".to_string())
    }

    pub fn cmd(&self) -> String {
        format!("{} {}", self.0, self.1)
    }
}

impl ShellCommand for GitStatus {
    type Cmd = String;
}

pub struct GitAdd(String, String);

impl GitAdd {
    pub fn new() -> Self {
        GitAdd("git".to_string(), "add".to_string())
    }

    pub fn cmd(&self) -> String {
        format!("{} {}", self.0, self.1)
    }
}
