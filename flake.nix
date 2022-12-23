{
  description = "zknotes, a web based zettelkasten";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nmattia/naersk";
  };

  outputs = { self, nixpkgs, flake-utils, naersk }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pname = "docx-template";
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        rust-stuff = naersk-lib.buildPackage {
            pname = pname;
            root = ./.;
            buildInputs = with pkgs; [
              cargo
              rustc
              pkgconfig
              ];
          };
      in
        rec {
          inherit pname;
          # `nix build`
          packages.${pname} = pkgs.stdenv.mkDerivation {
            nativeBuildInputs = [ pkgs.makeWrapper ];
            name = pname;
            src = ./.;
            # building the 'out' folder
            # installPhase = ''
            #   mkdir -p $out/share/zknotes/static
            #   mkdir $out/bin
            #   cp -r $src/server/static $out/share/zknotes
            #   cp ${elm-stuff}/main.js $out/share/zknotes/static
            #   cp -r ${rust-stuff}/bin $out
            #   mv $out/bin/zknotes-server $out/bin/.zknotes-server
            #   makeWrapper $out/bin/.zknotes-server $out/bin/zknotes-server --set ZKNOTES_STATIC_PATH $out/share/zknotes/static;
            #   '';
          };
          defaultPackage = packages.${pname};

          # `nix run`
          apps.${pname} = flake-utils.lib.mkApp {
            drv = packages.${pname};
          };
          defaultApp = apps.${pname};

          # `nix develop`
          devShell = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              cargo
              cargo-watch
              rustc
              rustfmt
              rust-analyzer
              pkgconfig
            ];
          };
        }
    );
}

