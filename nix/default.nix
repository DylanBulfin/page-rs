{ config, lib, pkgs, ... }:

with lib;

let
  cfg = config.programs.page-rs;
  tomlFormat = pkgs.formats.toml { };
in {
  options = {
    programs.page-rs = {
      enable = mkEnableOption "page-rs";

      package = mkOption {
        type = types.package;
        default = pkgs.page-rs;
        defaultText = literalExpression "pkgs.page-rs";
        description = "The page-rs package to install.";
      };

      settings = mkOption {
        type = tomlFormat.type;
        default = { };
        example = literalExpression ''
          {
            move_left = "m";
            move_down = "n";
            move_up = "e";
            move_right = "i";
            exit = "q";
            search = "/";
            next_match = "k";
            prev_match = "K";
          }
        '';
        description = ''
          Configuration written to
          {file}`$XDG_CONFIG_HOME/page-rs/config.toml`
        '';
      };
    };
  };

  config = mkIf cfg.enable {
    home.packages = [ cfg.package ];

    xdg.configFile."alacritty/alacritty.toml" = lib.mkIf (cfg.settings != { }) {
      source = (tomlFormat.generate "alacritty.toml" cfg.settings).overrideAttrs
        (finalAttrs: prevAttrs: {
          buildCommand = lib.concatStringsSep "\n" [
            prevAttrs.buildCommand
            # TODO: why is this needed? Is there a better way to retain escape sequences?
            "substituteInPlace $out --replace '\\\\' '\\'"
          ];
        });
    };
  };
}