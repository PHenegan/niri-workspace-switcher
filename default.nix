{ rustPlatform, glib, pkg-config }: 
rustPlatform.buildRustPackage {
  name = "niri-workspace-switcher";
  src = ./.;
  buildInputs = [ glib ];
  nativeBuildInputs = [ pkg-config ];
  cargoLock.lockFile = ./Cargo.lock;
};
