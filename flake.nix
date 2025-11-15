{
  description = "granola-cli - CLI tool to extract granola data";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin"];

      perSystem = {
        config,
        self',
        inputs',
        pkgs,
        system,
        ...
      }: let
        toolchain = inputs'.fenix.packages.stable.toolchain;
      in {
        devShells.default = pkgs.mkShell {
          packages = [
            toolchain
            inputs'.fenix.packages.rust-analyzer
          ];

          shellHook = ''
            echo "granola-cli development environment"
            echo "Rust version: $(rustc --version)"
          '';
        };
      };
    };
}
