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
        lib = pkgs.lib;

        cargoToml = fromTOML (builtins.readFile ./Cargo.toml);
        name = cargoToml.package.name;

        crate = pkgs.rustPlatform.buildRustPackage {
          inherit name;
          src = lib.sources.sourceFilesBySuffices ./. [
            "Cargo.toml"
            "Cargo.lock"
            ".rs"
          ];
          cargoLock.lockFile = ./Cargo.lock;

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            systemd
          ];

          meta.mainProgram = name;
        };
      in
      {
        packages.default = crate;

        nixosModules.default = {
          systemd.services."fwd-autostart" = {
            enable = true;
            serviceConfig = {
              ExecStart = lib.getExe crate;
            };
            wantedBy = [ "multi-user.target" ];
          };
        };

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
