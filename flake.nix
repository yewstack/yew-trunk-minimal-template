{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.x86_64-linux;
        fenix-system = fenix.packages.x86_64-linux;
        rust-toolchain = (with fenix-system;
          combine [
            default.toolchain
            complete.rust-src
            targets.wasm32-unknown-unknown.latest.rust-std
          ]);
        rustPlatform =
          # pkgs.makeRustPlatform { inherit (fenix-system.minimal) cargo rustc; };
          pkgs.makeRustPlatform {
            rustc = rust-toolchain;
            cargo = rust-toolchain;
          };
      in {
        defaultPackage = rustPlatform.buildRustPackage {
          pname = "yew-trunk-minimal-template";
          version = "0.1.0";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            trunk
            wasm-bindgen-cli
            nodePackages.sass
            pkgconfig
          ];

          cargoLock = { lockFile = ./Cargo.lock; };

          buildPhase = "trunk build";
          installPhase = "cp -r dist $out";

          # TODO: Use $TMP instead of /build
          XDG_CACHE_HOME = "/build/cache";
        };
        devShell = pkgs.mkShell {
          name = "rust-env";
          src = ./.;

          nativeBuildInputs = (with pkgs; [
            rust-toolchain
            fenix-system.rust-analyzer
            wasm-bindgen-cli
            nodePackages.sass
            trunk
            zlib
            pkgconfig
          ]);

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath [ pkgs.zlib ]
            }"'';
        };
      });
}
