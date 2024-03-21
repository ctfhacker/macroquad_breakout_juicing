let

  orig_pkgs = import (fetchTarball("channel:nixpkgs-23.11-darwin")) {};

  rust-overlay = import(orig_pkgs.fetchFromGitHub {
    owner = "oxalica";
    repo = "rust-overlay";
    rev = "7a94fe7690d2bdfe1aab475382a505e14dc114a6";
    sha256 = "sha256-/DZsoPH5GBzOpVEGz5PgJ7vh8Q6TcrJq5u8FcBjqAfI=";
  });


  pkgs = orig_pkgs.extend rust-overlay;

  rust = pkgs.rust-bin.nightly.latest.default.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" ];
  };

in 

pkgs.mkShell rec {
  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = with pkgs; [ 
    rust

    rust-analyzer
    pkg-config

    # x11 game related pkgs
    libGL
    udev
    alsa-lib
    vulkan-loader
    libxkbcommon
    xorg.libX11
    xorg.libXcursor
    xorg.libXi
    xorg.libXrandr
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
