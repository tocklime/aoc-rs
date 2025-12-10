{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-src"];
            })
            bacon
            hyperfine
            cargo-flamegraph
            cargo-make
            bashInteractive
            llvmPackages.libclang
            z3
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          BINDGEN_EXTRA_CLANG_ARGS = ''
            -I${pkgs.llvmPackages.libclang.lib}/clang/17/include
            -I${pkgs.z3}/include
          '';
          PKG_CONFIG_PATH = pkgs.lib.makeSearchPath "lib/pkgconfig" [
            pkgs.llvmPackages.libclang
          ];
        };
      }
    );
}
