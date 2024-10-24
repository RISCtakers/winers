{
  clippy,
  rustfmt,
  callPackage,
  rust-analyzer,
  wineWow64Packages
}:
let
  mainPkg = callPackage ./default.nix { };
in
mainPkg.overrideAttrs (oa: {
  nativeBuildInputs = [
    clippy
    rustfmt
    rust-analyzer
    wineWow64Packages.stagingFull # This crate is tested against this Wine version
  ] ++ (oa.nativeBuildInputs or [ ]);
})
