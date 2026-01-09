{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = with pkgs; [
    ffmpeg.dev
    clang
  ];
  languages.rust = {
    enable = true;
    channel = "stable";
  };
  languages.nix.enable = true;

  enterShell = ''
    export PATH="$PATH:${config.env.LIBCLANG_PATH}"
  '';

  env = {
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib/";
  };
}
