{ inputs, ... }:
{
  imports = [ inputs.treefmt-nix.flakeModule ];
  perSystem = _: {
    # Auto formatters. This also adds a flake check to ensure that the
    # source tree was auto formatted.
    treefmt.config = {
      projectRootFile = "flake.nix";

      programs = {
        # Rust
        rustfmt.enable = true;

        # ASM
        asmfmt.enable = true;

        # Nix
        deadnix.enable = true;
        nixfmt.enable = true;
        statix.enable = true;
      };
    };
  };
}
