# nix/treefmt.nix
{ pkgs, rustToolchain, ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true; # Nix formatting
    rustfmt.enable = true; # Rust formatting
    taplo.enable = true; # TOML formatting (Cargo.toml)
  };
  settings.formatter.rustfmt.options = [
    "--edition"
    "2024"
  ];
}
