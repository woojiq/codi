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
    devShells = forAllSystems (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # Rust
          rust-bin.stable.latest.default
          rust-analyzer
          gdb
        ];
      };
    });
  };
}
