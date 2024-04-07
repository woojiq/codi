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
          extensions = [];
        };
      rust-nightly = pkgs.rust-bin.nightly."2024-02-01".default;

      cargoNightlyUtil = name:
        pkgs.writeShellScriptBin name ''
          export RUSTC="${rust-nightly}/bin/rustc";
          export CARGO="${rust-nightly}/bin/cargo";
          exec "${pkgs.${name}}/bin/${name}" "$@"
        '';

      cargoToml = builtins.fromTOML (builtins.readFile ../Cargo.toml);

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rust;
        rustc = rust;
      };

      nativeBuildInputs = [
        rust
        pkgs.mold
      ];

      useMoldLinker = "-C link-arg=-fuse-ld=mold";
    in {
      packages.default = rustPlatform.buildRustPackage {
        inherit nativeBuildInputs pname;

        version = cargoToml.workspace.package.version;
        src = ./..;
        cargoLock.lockFile = ../Cargo.lock;
        doCheck = false;

        RUSTFLAGS = "${useMoldLinker} -D warnings";
        RUSTDOCFLAGS = "-D warnings";
      };

      checks.default = self.packages.${system}.default.overrideAttrs (
        finalAttrs: previousAttrs: {
          nativeCheckInputs = [
            rust
            (cargoNightlyUtil "cargo-udeps")
          ];
          # Tests are performed in checkPhase:
          # https://github.com/NixOS/nixpkgs/blob/master/pkgs/build-support/rust/hooks/cargo-check-hook.sh
          doCheck = true;

          postCheck = ''
            cargo clippy --all-targets
            cargo fmt --check
            cargo doc

            cargo udeps --all-targets
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

              bashInteractive
            ]
            ++ nativeBuildInputs;
          shellHook = ''
            export RUSTFLAGS="${useMoldLinker}";
          '';
        };
        nightly = pkgs.mkShell {
          packages = [
            rust-nightly
          ];
        };
      };
    });
}
