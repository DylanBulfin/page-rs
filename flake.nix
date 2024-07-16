{
  description = "THE MIMBLEWIMBLE BLOCKCHAIN.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-24.05";
  };

  outputs = { self, nixpkgs, }:
    # let
    #   supportedSystems = [ "x86_64-linux" "x86_64-darwin" "aarch64-linux" "aarch64-darwin" ];
    #   forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    #   nixpkgsFor = forAllSystems (system: import nixpkgs { inherit system; });
    # in
    {
      overlays.default = final: prev:
        with final;
        {
          page-rs = pkgs.rustPlatform.buildRustPackage {
            pname = "page-rs";
            version = "0.1.0";
            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };
          };
        };

    };
}
