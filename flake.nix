{
  description = "Template for Holochain app development";

  inputs = {
    versions.url  = "github:holochain/holochain?dir=versions/0_3_rc";

    holochain-flake.url = "github:holochain/holochain/";
    holochain-flake.inputs.versions.follows = "versions";
    versions.inputs.holochain.url = "github:holochain/holochain/holochain-0.3.0-beta-dev.46";

    nixpkgs.follows = "holochain-flake/nixpkgs";
    flake-parts.follows = "holochain-flake/flake-parts";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake
      {
        inherit inputs;
      }
      {
        systems = builtins.attrNames inputs.holochain-flake.devShells;
        perSystem =
          { inputs'
          , config
          , pkgs
          , system
          , ...
          }: {
            devShells.default = pkgs.mkShell {
              inputsFrom = [ inputs'.holochain-flake.devShells.holonix ];
              packages = with pkgs; [
                pkgs.nodejs-18_x
                nodePackages.pnpm
              ];
            };
          };
      };
}
