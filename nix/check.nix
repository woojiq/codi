{
  pkgs,
  rust,
}: {
  run-clippy = pkgs.writeShellApplication {
    name = "run-clippy";
    runtimeInputs = [
      rust
    ];
    text = ''
      cargo clippy -- \
        -D clippy::all \
        -D clippy::correctness \
        -D clippy::suspicious \
        -D clippy::style \
        -D clippy::complexity \
        -D clippy::perf \
        -D clippy::pedantic \
        -D clippy::nursery \
        -D clippy::cargo \
        -A clippy::must_use_candidate \
    '';
  };
}
