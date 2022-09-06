let
  holonixRev = "ce97dec539399d1e671ef7a2ee89e16de0b95561";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_156";
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
