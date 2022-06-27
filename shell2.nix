with import ./nix/pkgs.nix {};

stdenv.mkDerivation rec {
  name = "starter";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    vulkan-loader
    xorg.libX11
    x11
    xorg.libXrandr
    xorg.libXcursor
    xorg.libXi
    SDL2
    SDL2_mixer
    SDL2_image
    SDL2_ttf
  ];
  shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
                pkgs.lib.makeLibraryPath [
                  udev
                  alsaLib
                  vulkan-loader
                ]
              }"'';
}
