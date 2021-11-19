let
  holonixPath = builtins.fetchTarball {
    url = "https://github.com/holochain/holonix/archive/d453dde541d48db2a03a0250170cb4160f2cb880.tar.gz";
    sha256 = "sha256:1hdap7cx5x2gjb837dnnqifngb7spqx94vrc778jsad014kfj9bc";
  };
  holonix = import (holonixPath) {
    includeHolochainBinaries = true;
    holochainVersionId = "custom";

    holochainVersion = {
     rev = "29d5c1fcd0290a62bde30a23e227be6c7cdeb276";  # .115
     sha256 = "07vmg5sr0np6jds4xmjyj5nns83l56qhy75f6c8z09b7hh55bn2l";
     cargoSha256 = "sha256:1y3lq58684zn18s4ya9v9y7513cm4d1wpvwa2kvh08jn0awyw5pp";
     bins = {
       holochain = "holochain";
       hc = "hc";
     };

     lairKeystoreHashes = {
        sha256 = "0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv";
        cargoSha256 = "1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4";
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
  ];
}
