{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/e94a377b1f84ecc90a8705031a012793a7fdc93b.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    include = {
        # making this explicit even though it's the default
        holochainBinaries = true;
    };

    holochainVersionId = "custom";

    holochainVersion = {
      rev = "681eee88d509eae3fb8ef8eee027a6403fc89871"; # .112
      sha256 = "1cxx6iyf7h7jzrvkklmhjkl235vc0m6gp2bqffy25s4481c5jyl6";
      cargoSha256 = "0miqj8bslfznb858idv47k9rklraizrf56n1n5w9mdzlwnzgvv1f";
      bins = {
        holochain = "holochain";
        hc = "hc";
        kitsune-p2p-proxy = "kitsune_p2p/proxy";
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
    binaryen sqlite.dev
  ];
}
