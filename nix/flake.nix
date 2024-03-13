{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    # TODO: Get rid of flake-utils: https://ayats.org/blog/no-flake-utils/
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachSystem ["x86_64-linux"] (system: let
      pkgs = nixpkgs.legacyPackages.${system}.extend (import rust-overlay);
      rust = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;

      cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);
      check = import ./check.nix {inherit pkgs rust;};

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      nativeBuildInputs = [
        rust
      ];
    in {
      packages.default = rustPlatform.buildRustPackage {
        inherit nativeBuildInputs;

        pname = cargoToml.package.name;
        version = cargoToml.package.version;
        src = ./..;
        cargoLock.lockFile = ../Cargo.lock;
        doCheck = false;
      };

      checks.default = self.packages.${system}.default.overrideAttrs (
        finalAttrs: previousAttrs: {
          nativeCheckInputs = [
            rust
            check.clippy-deny
          ];
          doCheck = true;
          checkPhase = ''
            cargo test
            run-clippy
          '';
        }
      );

      devShells = {
        default = pkgs.mkShell {
          packages = with pkgs;
            [
              rust-analyzer
              gdb
              check.clippy-warn
            ]
            ++ nativeBuildInputs;
        };
        nightly = pkgs.mkShell {
          packages = with pkgs; [
            rust-bin.nightly."2024-02-01".default
          ];
        };
      };
    });
}
