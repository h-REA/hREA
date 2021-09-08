{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/62659a81d9f2d25740c2b2f424b90f50a60eb80c.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    include = {
        # making this explicit even though it's the default
        holochainBinaries = true;
    };

    holochainVersionId = "custom";

    holochainVersion = {
      rev = "d003eb7a45f1d7125c4701332202761721793d68"; # .104
      sha256 = "0qxadszm2a7807w49kfbj7cx6lr31qryxcyd2inyv7q5j7qbanf2";
      cargoSha256 = "129wicin99kmxb2qwhll8f4q78gviyp73hrkm6klpkql6810y3jy";
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
