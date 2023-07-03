{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [(import rust-overlay)];
      };

      rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      craneLib = (crane.mkLib pkgs).overrideToolchain rustTarget;
      git-view = craneLib.buildPackage {
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        buildInputs = [] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [pkgs.libiconv];
      };
    in {
      checks = {
        inherit git-view;
      };

      packages.default = git-view;
      apps.default = flake-utils.lib.mkApp {
        drv = git-view;
      };

      devShells.default = pkgs.mkShell {
        name = "rust-shell";
        inputsFrom = builtins.attrValues self.checks.${system};
        nativeBuildInputs = with pkgs; [
          cargo
          rustc
        ];
      };
    });
}
