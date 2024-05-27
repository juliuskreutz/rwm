{
  description = "Rwm";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
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
      nixosModules.default = {
        config = {
          services.xserver.windowManager.session = [
            {
              name = "rwm";
              start = ''
                       rwm &
                waitPID=$!
              '';
            }
          ];
        };
      };
    }
    // (flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};

      craneLib = crane.mkLib pkgs;

      commonArgs = {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
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

      rwm = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        });
    in {
      checks = {
        inherit rwm;
      };

      formatter = pkgs.alejandra;

      packages.default = rwm;
      packages.rwm = rwm;

      apps.default = flake-utils.lib.mkApp {
        drv = rwm;
      };
    }));
}
