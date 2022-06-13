{
  description = "Generates color palettes from images for use with your UI";

  inputs = {
    naersk.url = "github:nmattia/naersk/master";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };
          naersk-lib = naersk.lib."${system}";
          nativeBuildInputs = builtins.attrValues { inherit (pkgs) cargo cargo-watch rustc rustfmt clippy pkg-config; };
          buildInputs = [ ];
        in
        {
          defaultPackage = naersk-lib.buildPackage {
            pname = "arpeggio";
            root = builtins.path {
              path = ./.;
              name = "arpeggio-src";
            };
            inherit nativeBuildInputs buildInputs;
          };

          defaultApp = utils.lib.mkApp {
            drv = self.defaultPackage."${system}";
          };

          devShell = pkgs.mkShell {
            nativeBuildInputs = nativeBuildInputs ++ buildInputs;
          };
        }) // {
      overlay = final: prev: {
        arpeggio = self.defaultPackage.${prev.system};
      };
    };
}
