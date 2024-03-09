{
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  }: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
      ] (system: function (nixpkgs.legacyPackages.${system}.extend (import rust-overlay)));
  in {
    devShells = forAllSystems (pkgs: let
      rust_ = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
      check = import ./check.nix {inherit pkgs rust_;};
    in {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rust_
          rust-analyzer
          gdb
          check.check-clippy
        ];
      };
      nightly = pkgs.mkShell {
        packages = with pkgs; [
          rust-bin.nightly."2024-02-01".default
        ];
      };
    });
  };
}
