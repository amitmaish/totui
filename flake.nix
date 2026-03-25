{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ {flake-parts, ...}:
    flake-parts.lib.mkFlake {inherit inputs;} ({
      config,
      withSystem,
      moduleWithSystem,
      ...
    }: {
      systems = inputs.nixpkgs.lib.platforms.unix;
      perSystem = {
        system,
        config,
        pkgs,
        ...
      }: {
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [inputs.rust-overlay.overlays.default];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            just
          ];

          buildInputs = with pkgs; [
            rust-bin.stable.latest.default
            rust-bin.stable.latest.rust-analyzer
          ];
        };
      };
    });
}
