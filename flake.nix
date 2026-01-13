{
  description = "{{NAME}}";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      flake-utils,
      nixpkgs,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        inputs = with pkgs; [
          openssl
          pkg-config
          xorg.libX11
          xorg.libX11.dev
          xorg.libXi
          xorg.libXi.dev
          xorg.libICE
          xorg.libSM
          xorg.libXinerama
          fontconfig
          libGL
          libxkbcommon
          glib
          freetype
        ];
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            buildInputs = [
              (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            ]
            ++ inputs;

            LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath inputs}";

            # LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            #   pkgs.freetype
            #   pkgs.libGL
            #   pkgs.xorg.libX11
            #   pkgs.xorg.libXi
            #   pkgs.xorg.libXrandr
            #   libxkbcommon
            # ];

            shellHook = '''';
          };
      }
    );
}
