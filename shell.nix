{
  clippy,
  rustfmt,
  callPackage,
  rust-analyzer,
}: let
  mainPkg = callPackage ./default.nix {};
in
  mainPkg.overrideAttrs (oa: {
    buildInputs =
      [
        clippy
        rustfmt
        rust-analyzer
      ]
      ++ (oa.buildInputs or [])
      ++ (oa.nativeBuildInputs or []);
  })
