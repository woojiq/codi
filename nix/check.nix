{
  pkgs,
  rust_,
}: {
  check-clippy = pkgs.writeShellApplication {
    name = "check-clippy";
    runtimeInputs = [
      rust_
    ];
    text = ''
      cargo clippy -- \
        -W clippy::all \
        -W clippy::correctness \
        -W clippy::suspicious \
        -W clippy::style \
        -W clippy::complexity \
        -W clippy::perf \
        -W clippy::pedantic \
        -W clippy::nursery \
        -W clippy::cargo \
        -A clippy::must_use_candidate
    '';
  };
}
