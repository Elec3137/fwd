{
  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixos-unstable/nixexprs.tar.zst";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    inputs:
    let
      lib = inputs.nixpkgs.lib;

      src = lib.sources.sourceFilesBySuffices ./. [
        "Cargo.toml"
        "Cargo.lock"
        ".rs"
      ];

      cargoToml = fromTOML (builtins.readFile "${src}/Cargo.toml");
      name = cargoToml.package.name;

      bin =
        {
          rustPlatform,
          pkg-config,
          systemd,
        }:
        rustPlatform.buildRustPackage {
          inherit name src;
          cargoLock.lockFile = "${src}/Cargo.lock";
          nativeBuildInputs = [
            pkg-config
          ];
          buildInputs = [
            systemd
          ];

          meta.mainProgram = name;
        };
    in
    inputs.flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = inputs.nixpkgs.legacyPackages.${system};
        default = pkgs.callPackage bin { };
      in
      {
        packages.default = default;

        devShells.default = pkgs.mkShell {
          inputsFrom = [ default ];

          packages = with pkgs; [
            clippy
            rustfmt
            rust-analyzer
          ];
        };
      }
    )
    // {
      nixosModules.default = { pkgs, ... }: {
        systemd.services."fwd-autostart" = {
          serviceConfig = {
            ExecStart = lib.getExe (inputs.nixpkgs.legacyPackages.${pkgs.stdenv.system}.callPackage bin { });
          };
          wantedBy = [ "multi-user.target" ];
        };
      };
    };
}
