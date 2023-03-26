{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "cloudfetch";
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    # Add any other dependencies your project needs
  ];

  # Set any environment variables needed for your project
  # Example:
  # CARGO_BUILD_TARGET_DIR = "target";
}
