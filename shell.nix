with (import <nixpkgs> {});
stdenv.mkDerivation {
  name = "nix-drv-shell-env";
  buildInputs = with pkgs; [ rustc cargo ];
}
