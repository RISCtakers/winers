{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };

  outputs = {nixpkgs, ...}: let
    forAllSystems = function:
      nixpkgs.lib.genAttrs nixpkgs.lib.systems.flakeExposed (
        system: function nixpkgs.legacyPackages.${system}
      );
  in {
    packages = forAllSystems (pkgs: rec {
      winers = pkgs.callPackage ./default.nix {};
      default = winers;
    });

    devShells = forAllSystems (pkgs: {
      default = pkgs.callPackage ./shell.nix {};
    });

    overlays.default = final: _: {winers = final.callPackage ./default.nix {};};

    formatter = forAllSystems (pkgs: pkgs.alejandra);
  };
}
