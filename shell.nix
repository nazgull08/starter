with import ./nix/pkgs.nix {};

stdenv.mkDerivation rec {
  name = "starter";
  env = buildEnv { name = name; paths = buildInputs; };

  buildInputs = [
    rustup
    SDL2
    SDL2_mixer
  ];
}
