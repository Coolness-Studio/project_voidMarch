{
  description = "Dev shell for project_voidmarch";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

      runtimeLibs = with pkgs; [
        libx11
        libxcursor
        libxi
        libxrandr
        libGL
      ];
    in {
      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          rustc
          cargo
          rust-analyzer
          clippy
          rustfmt
          pkg-config
        ] ++ runtimeLibs;

        shellHook = ''
          export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath runtimeLibs}:$LD_LIBRARY_PATH"
        '';
      };
    };
}
