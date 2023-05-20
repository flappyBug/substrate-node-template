{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay  = {
        url = "github:oxalica/rust-overlay";
        inputs = {
            nixpkgs.follows = "nixpkgs";
            flake-utils.follows = "flake-utils";
        };
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            exa
            fd
            protobuf
            clang
            llvmPackages.libclang
            rustToolchain
          ];

          shellHook = ''
            alias ls=exa
            alias find=fd
          '';
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
        };
      }
    );
}
