{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
  };
  outputs = {self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
      {
        packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
          pname = "clinews";
          version = "1.0.0";
          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
        devShells.${system}.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
          ];
        };
      };
}
