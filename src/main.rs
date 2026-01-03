use clap::Parser;
use niri_ipc::{Action, Request, Response, Workspace, WorkspaceReferenceArg, socket::Socket};

#[derive(Parser)]
struct Cli {
    workspace: String,
}

struct NiriState {
    socket: Socket,
    workspaces: Vec<Workspace>,
}

type Error = Box<dyn std::error::Error + 'static>;

impl NiriState {
    fn new() -> Result<Self, Error> {
        let mut socket = Socket::connect()?;
        let workspaces = NiriState::get_workspaces(&mut socket)?;

        Ok(NiriState { socket, workspaces })
    }

    fn get_workspaces(niri_socket: &mut Socket) -> Result<Vec<Workspace>, Error> {
        let reply = niri_socket.send(Request::Workspaces)??;
        match reply {
            Response::Workspaces(workspaces) => Ok(workspaces),
            _ => unreachable!("Should exclusively respond with workspaces response"),
        }
    }

    fn switch_to_workspace(&mut self, workspace: String) -> Result<(), Error> {
        // workspace which the user currently has focused
        let current = self
            .workspaces
            .iter()
            .find(|workspace| workspace.is_focused)
            .unwrap(); // TODO: Make an Error Enum
        let target = self.find_target_workspace(&workspace)?.clone();

        let outputs_match = current.output.eq(&target.output);

        if !outputs_match && !target.is_active {
            // If the target is on a different monitor but isn't being displayed, just move it over
            self.socket
                .send(Request::Action(Action::MoveWorkspaceToMonitor {
                    output: current.output.as_ref().unwrap_or(&"".to_string()).clone(),
                    reference: Some(WorkspaceReferenceArg::Id(target.id)),
                }))??;
        } else if !outputs_match {
            // If the target and current workspaces are both active, swap them
            self.socket
                .send(Request::Action(Action::MoveWorkspaceToMonitor {
                    output: target.output.as_ref().unwrap_or(&"".to_string()).clone(),
                    reference: Some(WorkspaceReferenceArg::Id(current.id)),
                }))??;
            self.socket
                .send(Request::Action(Action::MoveWorkspaceToMonitor {
                    output: current.output.as_ref().unwrap_or(&"".to_string()).clone(),
                    reference: Some(WorkspaceReferenceArg::Id(target.id)),
                }))??;

            // Focus the current workspace on the other monitor before focusing the target one
            self.socket.send(Request::Action(Action::FocusWorkspace {
                reference: WorkspaceReferenceArg::Id(current.id),
            }))??;
        }

        self.socket.send(Request::Action(Action::FocusWorkspace {
            reference: WorkspaceReferenceArg::Id(target.id),
        }))??;

        Ok(())
    }

    fn find_target_workspace(&self, target: &String) -> Result<&Workspace, Error> {
        let named_workspace = self.workspaces.iter().find(|workspace| {
            workspace
                .name
                .as_ref()
                .is_some_and(|name| name.eq_ignore_ascii_case(target))
        });
        match named_workspace {
            // TODO: This is atrocious
            Some(workspace) => Ok(workspace),
            None => {
                let id: u64 = target.parse()?;
                Ok(self
                    .workspaces
                    .iter()
                    .find(|workspace| workspace.id == id)
                    // If the workspace at the given index does not exist, just use the highest one
                    .unwrap_or_else(|| {
                        self.workspaces
                            .iter()
                            .max_by(|a, b| a.idx.cmp(&b.idx))
                            .unwrap()
                    }))
            }
        }
    }
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let mut state = NiriState::new()?;

    state.switch_to_workspace(args.workspace)?;
    Ok(())
}
