{
  clippy,
  rustfmt,
  callPackage,
  rust-analyzer,
  wineWow64Packages
}: let
  mainPkg = callPackage ./default.nix {};
in
  mainPkg.overrideAttrs (oa: {
    buildInputs =
      [
        clippy
        rustfmt
        rust-analyzer
        wineWow64Packages.stagingFull
      ]
      ++ (oa.buildInputs or [])
      ++ (oa.nativeBuildInputs or []);
  })
