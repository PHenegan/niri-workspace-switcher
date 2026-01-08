mod error;
mod sort;
mod switch;

use clap::Parser;
use niri_ipc::socket::Socket;
use niri_ipc::{Request, Response, Workspace};

type Error = Box<dyn std::error::Error + 'static>;

#[derive(Parser)]
struct Cli {
    workspace: String,
}

struct NiriState {
    socket: Socket,
    workspaces: Vec<Workspace>,
}

impl NiriState {
    pub fn new() -> Result<Self, Error> {
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
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let mut state = NiriState::new()?;

    state.switch_to_workspace(args.workspace)?;
    state.sort_workspaces()?;
    Ok(())
}
