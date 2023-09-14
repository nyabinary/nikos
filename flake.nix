{
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixos-unstable";
    };
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [
          qemu
          cargo-bootimage
          lldb
          (fenix.packages.${system}.complete.withComponents [
            "rustc"
            "cargo"
            "rustfmt"
            "rust-analyzer"
            "clippy"
            "miri"
            "rust-src"
            "llvm-tools"
          ])
        ];
      };
    });
}
