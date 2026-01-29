{
  description = "Flake for building github:PHenegan/niri-workspace-switcher";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }: let
    lib = nixpkgs.lib;

    # Creates a list of architecture strings (as opposed to hardcoding "x86_64-linux", "aarch64-linux", etc.)
    # This `systems` list was copied directly from the Niri flake, so it should support all of the same platforms
    systems = lib.intersectLists lib.systems.flakeExposed lib.platforms.linux;

    # lib.genAttrs converts list of strings into an object with list items as keys
    systemPackages = lib.genAttrs systems (
      system: nixpkgs.legacyPackages.${system}
    );
  in {

    # Map entries in systemPackages to devShells.{system} objects
    # lib.mapAttrs converts an object's values from one format to another
    # `system` represents a field in `systemPackages`, while `pkgs` represents its original value in `systemPackages`
    devShells = lib.mapAttrs (
      system: pkgs: {
        buildInputs = with pkgs; [
          cargo rustc rustfmt clipply rust-analyzer glib
        ];
        nativeBuildInputs = [ pkgs.pkg-config ];
        
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      }
    ) systemPackages;

    # Map entries in systemPackages to output packages.{system} entries
    packages = lib.mapAttrs (
      system: pkgs: {
        default = pkgs.callPackage ./default.nix { };
      }
    ) systemPackages;
  };
}
