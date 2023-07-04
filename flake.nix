{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
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
          # The `postprocessors` key hasn't been released yet.
          (git-cliff.overrideAttrs
            (drv: rec {
              pname = "git-cliff";
              version = "9692ea7e317d472e2fb352abb64abca4116ef93f";

              src = pkgs.fetchFromGitHub {
                owner = "orhun";
                repo = "git-cliff";
                rev = "${version}";
                sha256 = "sha256-OhvsSMOnTULpRhUuMOWClQgrIM/LN+89LM0vAU24I0o=";
              };

              cargoDeps = drv.cargoDeps.overrideAttrs (lib.const {
                name = "${pname}-${version}-vendor.tar.gz";
                inherit src;
                outputHash = "sha256-2ZJZi78K2nkkCdRyBL+AJARQjesRciYvHDh1ibYRuM8=";
              });
            }))
        ];
      };
    });
}
