# niri-workspace-switcher
(name pending)

This is a simple program which communicates over Niri's unix socket to make managing workspaces match my personal workflows.
It implements the following features:

- When a user switches to a named workspace, it will be brought to the monitor being currently focused.
- If the user switches to a workspace which is currently active on another monitor, the current workspace will swap to the other monitor
- After moving workspaces, all workspaces on all monitors will be sorted by ID. This means that named workspaces will always be listed in the order in which they were defined in the Niri configuration file(s).

This is my attempt to match the behavior implemented by Qtile/XMonad, since I frequently move workspaces back and forth between my 2 monitors. I also switched to Niri from Qtile,
so part of it is just not wanting to give up my muscle memory. I implemented the sorting feature purely because my hotkeys for named workspaces still use numbers
and it was bothering me seeing them out of order in waybar.

## Usage
`niri-workspace-switcher [workspace-name]` will focus the intended workspace using the behavior described above.
`niri-workspace-switcher [index]` will focus the specified index on the current monitor, or the highest index if that one does not exist

## Building
You can build this tool using `cargo build --release` to get a resulting binary. There is nothing architecture specific about this project, so it should work on any linux platform which Niri supports.
You will have to move the resulting binary (`./target/release/niri-workspace-switcher`) into a location present on your `$PATH` variable to use this application with Niri.

If you have Nix installed, you can also do `nix build`, which will create a symlinked binary called `result` in this project's root directory.
For now this mode of building is only supported on `x86_64` because all of the flake guides I could find only specified `x86_64`. The project should support ARM systems though so I will try to add this in the future.

## Packaging
The only packaging method I officially maintain is the Nix flake present in this repository. Feel free to package this elsewhere (e.g. AUR), but the only "official" source I will maintain is this repository.

## Versioning
Right now I consider this beta software so things might change (especially the name). Most of the behavior should stay roughly the same, though I might slightly change the CLI syntax if I add more commands to this project.
Rewrites/Refactoring aside, I don't want to push anything which fully breaks behavior to the `main` branch.
