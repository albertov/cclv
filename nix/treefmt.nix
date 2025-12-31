# nix/treefmt.nix
{ pkgs, ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true; # Nix formatting
    rustfmt.enable = true; # Rust formatting
    taplo.enable = true; # TOML formatting (Cargo.toml)
  };
}
