{
  description = "Flake for Holochain app development";

  inputs = {
    holonix.url = "github:holochain/holonix?ref=main-0.6";

    nixpkgs.follows = "holonix/nixpkgs";
    flake-parts.follows = "holonix/flake-parts";
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = builtins.attrNames inputs.holonix.devShells;
    perSystem = { inputs', pkgs, ... }: {
      formatter = pkgs.nixpkgs-fmt;

      devShells.default = pkgs.mkShell {
        inputsFrom = [ inputs'.holonix.devShells ];

        packages = (with inputs'.holonix.packages; [
          holochain
          hc
          bootstrap-srv
          lair-keystore
          hc-scaffold
          hn-introspect
          rust # For Rust development, with the WASM target included for zome builds
        ]) ++ (with pkgs; [
          nodejs_22 # For UI development
          yarn # Package manager for Node.js
          binaryen # For WASM optimisation
          typescript # TypeScript compiler for GraphQL adapter
          # Add any other packages you need here
        ]);

        shellHook = ''
          export PS1='\[\033[1;34m\][holonix:\w]\$\[\033[0m\] '
        '';
      };
    };
  };
}