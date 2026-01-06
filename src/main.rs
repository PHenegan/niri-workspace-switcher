mod error;
mod switch;

use clap::Parser;
use niri_ipc::Workspace;
use niri_ipc::socket::Socket;

type Error = Box<dyn std::error::Error + 'static>;

#[derive(Parser)]
struct Cli {
    workspace: String,
}

struct NiriState {
    socket: Socket,
    workspaces: Vec<Workspace>,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let mut state = NiriState::new()?;

    state.switch_to_workspace(args.workspace)?;
    Ok(())
}
