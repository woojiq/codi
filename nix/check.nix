{
  pkgs,
  rust,
}: let
  clippyCommonText = ''
    cargo clippy --all -- \
      -W clippy::all \
      -W clippy::correctness \
      -W clippy::suspicious \
      -W clippy::style \
      -W clippy::complexity \
      -W clippy::perf \
      -W clippy::pedantic \
      -W clippy::nursery \
      -W clippy::cargo \
      -A clippy::must_use_candidate \
      -A clippy::suboptimal_flops \
  '';
  clippyWithText = text:
    pkgs.writeShellApplication {
      name = "run-clippy";
      runtimeInputs = [
        rust
      ];
      inherit text;
    };
in {
  # The reason we have different packages for "warn" and "deny" is that if you set
  # the level to "deny" you won't see all the errors in one run.
  clippy-warn = clippyWithText clippyCommonText;
  clippy-deny = clippyWithText ''
    ${clippyCommonText} -D warnings
  '';
}
