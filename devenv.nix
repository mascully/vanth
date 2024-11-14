{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

let
  pkgs-unstable = import inputs.nixpkgs-unstable {
    system = pkgs.stdenv.system;
    config.allowUnfree = true;
  };
in
{
  # https://devenv.sh/basics/
  env.GREET = "devenv";

  # https://devenv.sh/packages/
  packages = [
    pkgs-unstable.git
    pkgs-unstable.rustup
    pkgs-unstable.code-cursor
    pkgs-unstable.curl
    pkgs-unstable.jq
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    channel = "nightly";
    components = [
      "cargo"
      "rust-src"
      "rustc"
      "clippy"
    ];
    enable = true;
  };

  enterShell = ''
    rustup component add clippy
  '';

  enterTest = ''
    echo "Running tests"
    git --version | grep --color=auto "${pkgs.git.version}"
    cargo clippy -- -D warnings
  '';

  pre-commit.hooks = {
    shellcheck.enable = true;
    clippy = {
      enable = true;
      name = "clippy";
      entry = "cargo clippy -- -D warnings";
      files = "\\.rs$";
      language = "system";
    };
  };
}
