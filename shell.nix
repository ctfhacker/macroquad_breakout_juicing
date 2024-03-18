let

  orig_pkgs = import (fetchTarball("channel:nixpkgs-23.11-darwin")) {};

  rust-overlay = import(orig_pkgs.fetchFromGitHub {
    owner = "oxalica";
    repo = "rust-overlay";
    rev = "7a94fe7690d2bdfe1aab475382a505e14dc114a6";
    sha256 = "sha256-/DZsoPH5GBzOpVEGz5PgJ7vh8Q6TcrJq5u8FcBjqAfI=";
  });

  pkgs = orig_pkgs.extend rust-overlay;

in 

pkgs.mkShell rec {
  nativeBuildInputs = [
    pkgs.pkg-config
  ];

  buildInputs = [ 
    pkgs.rust-bin.nightly.latest.default
    pkgs.rust-analyzer
    pkgs.pkg-config

    pkgs.libGL
    pkgs.udev
    pkgs.alsa-lib
    pkgs.vulkan-loader
    pkgs.libxkbcommon
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.xorg.libXi
    pkgs.xorg.libXrandr
  ];

  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
