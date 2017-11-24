cargo build
for pkg in firefox bash mcrl2; do
  drv_path=$(nix-instantiate -E "with import <nixpkgs> {}; pkgs.$pkg")
  echo; echo "$pkg: $drv_path"
  target/debug/prettify-nix-drv $drv_path
done
