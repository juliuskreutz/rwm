{
  description = "Rwm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = inputs @ {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    {
      overlays.default = _: prev: {
        rwm = self.packages.${prev.stdenv.hostPlatform.system}.default;
      };
      overlays.rwm = self.overlays.default;

      nixosModules.default = import ./rwm.nix inputs;
      nixosModules.rwm = self.nixosModules.default;
    }
    // (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = crane.mkLib pkgs;
      src = craneLib.cleanCargoSource ./.;

      commonArgs = {
        inherit src;
        strictDeps = true;

        nativeBuildInputs = with pkgs; [
          pkg-config
          rustPlatform.bindgenHook
        ];

        buildInputs = with pkgs; [
          xcb-util-cursor
          glib
          pango
        ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      rwm = craneLib.buildPackage (commonArgs
        // {
          inherit cargoArtifacts;
        });
    in {
      checks = {
        inherit rwm;
      };

      packages.default = rwm;
      packages.rwm = self.packages.default;

      apps.default = flake-utils.lib.mkApp {
        drv = rwm;
      };

      devShells.default = craneLib.devShell {
        checks = self.checks.${system};

        packages = with pkgs; [
          rust-analyzer
          taplo
        ];
      };
    }));
}
