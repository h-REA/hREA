let
  holonixRev = "0b1bf0eb19626f354c8efce70a21dbdbbaffbc71";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_161";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = with nixpkgs; [
    # :TODO: binaryen, wasm-opt?
    # Additional packages go here
    nodejs-16_x
    nodePackages.pnpm
  ];
}
