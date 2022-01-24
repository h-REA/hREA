let
  holonixRev = "2f7b8047d6314f64fca34394a52d465c18b2f4d5";

  holonixPath = builtins.fetchTarball "https://github.com/holochain/holonix/archive/${holonixRev}.tar.gz";
  holonix = import (holonixPath) {
    holochainVersionId = "v0_0_123";
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  packages = [
    # :TODO: binaryen, wasm-opt?
  ];
}
