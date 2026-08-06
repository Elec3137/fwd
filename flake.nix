{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import inputs.nixpkgs { inherit system; };

        cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
        name = cargoToml.package.name;

        crate = pkgs.rustPlatform.buildRustPackage {
          inherit name;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            systemd
          ];
        };
      in
      {
        packages.default = crate;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ crate ];

          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      }
    );
}
