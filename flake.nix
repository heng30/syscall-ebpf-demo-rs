{
  description = "A very basic flake for Rust ebpf engine";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };

      shellBuildInputs = with pkgs; [
        glibc.dev
        libGL.dev
        stdenv.cc
        gcc-unwrapped.lib
        libbpf
      ];

      rustToolchain = pkgs.rust-bin.nightly.latest.default.override {
        extensions = [ "rust-src" ];
        targets = [ ];
      };
      rustSrc = rustToolchain + "/lib/rustlib/src/rust";
    in {
      devShells.${system}.default = pkgs.mkShell {
        nativeBuildInputs = with pkgs; [
          gcc
          llvm
          clang
          python3
          gnumake
          pkg-config
          rustToolchain
          rust-analyzer
          bpftools
        ];

        buildInputs = shellBuildInputs;

        env = {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath shellBuildInputs;
          C_INCLUDE_PATH = "${pkgs.glibc.dev}/include";
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          RUST_SRC_PATH = rustSrc;

          CPLUS_INCLUDE_PATH = with pkgs;
            lib.concatStringsSep ":" [
              "${gcc}/include/c++/${gcc.version}"
              "${gcc}/include/c++/${gcc.version}/x86_64-unknown-linux-gnu"
              "${glibc.dev}/include"
            ];

          PKG_CONFIG_PATH = with pkgs;
            lib.concatStringsSep ":" [
              # "${wayland.dev}/lib/pkgconfig"
            ];
        };

        shellHook = ''
          export PS1="($(basename $(pwd)))> ";
          alias ee=exit
          cargo install bpf-linker # --force

          echo "[INFO] ✨ Rust development environment with epbf engine support is ready!"
          echo "[INFO] 📦 Rust version: $(rustc --version)"
          echo "[INFO] 📦 C++ compiler: $(g++ --version | head -n1)"
          echo "[INFO] 📦 Clang version: ${pkgs.llvmPackages.clang.version}"
          echo "[INFO] 📦 Rust source available at: $RUST_SRC_PATH"
        '';
      };
    };
}
