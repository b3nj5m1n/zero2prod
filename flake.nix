{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs = {
        flake-utils.follows = "flake-utils";
        nixpkgs.follows = "nixpkgs";
      };
    };
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    fenix,
    crane,
  }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in rec {
        packages = flake-utils.lib.flattenTree {
          zero2prod =
            let
              craneLib = crane.lib.${system}.overrideToolchain
                fenix.packages.${system}.minimal.toolchain;
            in
              craneLib.buildPackage {
                nativeBuildInputs = with pkgs; [pkg-config openssl];
                src = ./.;
              };
        };
        defaultPackage = packages.zero2prod;
        apps = {
          zero2prod = flake-utils.lib.mkApp { drv = packages.zero2prod; };
        };
        defaultApp = apps.zero2prod;
      }
    );
}
