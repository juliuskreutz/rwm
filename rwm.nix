inputs: {
  config,
  lib,
  pkgs,
  ...
}: let
  cfg = config.services.xserver.windowManager.rwm;
in {
  options.services.xserver.windowManager.rwm = {
    enable = lib.mkEnableOption "rwm";
    package = lib.mkPackageOption pkgs "rwm" {};
  };

  config = lib.mkIf cfg.enable {
    nixpkgs.overlays = [inputs.self.overlays.default];

    services.xserver.windowManager.session =
      lib.singleton
      {
        name = "rwm";
        start = ''
          ${pkgs.rwm}/bin/rwm &
          waitPID=$!
        '';
      };
  };
}
