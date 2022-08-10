let
  holonixRev = "3fa45915b9a323d16d899e9e82d27b04314523e6";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_152";
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
