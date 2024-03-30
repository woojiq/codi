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
      pname = "codi";
      pkgs = nixpkgs.legacyPackages.${system}.extend (import rust-overlay);
      rust =
        (pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml)
        .override {
          targets = [];
        };

      cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      nativeBuildInputs = [
        rust
      ];
    in {
      packages.default = rustPlatform.buildRustPackage {
        inherit nativeBuildInputs pname;

        version = cargoToml.workspace.package.version;
        src = ./..;
        cargoLock.lockFile = ../Cargo.lock;
        doCheck = false;
      };

      checks.default = self.packages.${system}.default.overrideAttrs (
        finalAttrs: previousAttrs: {
          nativeCheckInputs = [
            rust
          ];
          # https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/rust/hooks/cargo-check-hook.sh
          doCheck = true;
          postCheck = ''
            cargo clippy --workspace --all-targets -- -D warnings
            cargo fmt --check
          '';
        }
      );

      apps.default = {
        type = "app";
        program = "${self.packages.${system}.default}/bin/${pname}";
      };

      devShells = {
        default = pkgs.mkShell {
          packages = with pkgs;
            [
              rust-analyzer
              gdb
              cargo-mutants
              cargo-tarpaulin

              hyprpicker
              bashInteractive
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
