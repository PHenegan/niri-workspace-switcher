use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum SwitchWorkspaceError {
    NoFocusedOutput,
    NoTargetWorkspace,
}

impl Display for SwitchWorkspaceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for SwitchWorkspaceError {}
