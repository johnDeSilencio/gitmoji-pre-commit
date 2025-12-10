{
  description = "Nix-flake development environment for Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    flake-utils.url = "github:numtide/flake-utils";

    crane = {
      url = "github:ipetkov/crane";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    wild = {
      url = "github:davidlattimore/wild";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      flake-utils,
      rust-overlay,
      wild,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            (import rust-overlay)
            wild.overlays.default
          ];
        };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p: p.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml
        );

        gitmoji-pre-commit = craneLib.buildPackage {
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          doCheck = true;
          doNotSign = false;
        };
      in
      {
        checks = {
          inherit gitmoji-pre-commit;
        };

        packages.default = gitmoji-pre-commit;

        devShells.default = craneLib.devShell {
          # Inherit inputs from checks
          checks = self.checks.${system};

          packages = [
            pkgs.clang
            pkgs.openssl
            pkgs.pkg-config
            pkgs.wild
          ];
        };
      }
    );
}
