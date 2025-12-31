# nix/treefmt.nix
{ pkgs, myRustToolchain, ... }:
{
  projectRootFile = "flake.nix";
  programs = {
    nixfmt.enable = true; # Nix formatting
    rustfmt.enable = true; # Rust formatting
    rustfmt.package = myRustToolchain;
    taplo.enable = true; # TOML formatting (Cargo.toml)
  };
  /*
    settings.formatter.rustfmt.options = [
      "--edition"
      "2024"
    ];
  */
}
