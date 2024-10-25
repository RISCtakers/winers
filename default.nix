{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
}: let
  toml = (lib.importTOML ./Cargo.toml).package;
in
  rustPlatform.buildRustPackage rec {
    pname = toml.name;
    inherit (toml) version;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.intersection (lib.fileset.fromSource (lib.sources.cleanSource ./.)) (
        lib.fileset.unions [
          ./Cargo.toml
          ./Cargo.lock
          ./src
        ]
      );
    };

    buildInputs = [
      openssl
    ];

    nativeBuildInputs = [
      pkg-config
    ];

    cargoLock.lockFile = ./Cargo.lock;

    meta.mainProgram = pname;
  }
