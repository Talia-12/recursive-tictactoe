{ pkgs }: pkgs.mkShell rec {
  # Get dependencies from the main package
  inputsFrom = [(pkgs.callPackage ./default.nix {})];
  # Additional tooling
  buildInputs = with pkgs; [
    rust-analyzer # LSP Server
    rustfmt # Formatter
    clippy # Linter

    libxkbcommon
    vulkan-loader
  ];

  LD_LIBRARY_PATH    = pkgs.lib.makeLibraryPath buildInputs;
  WINIT_UNIX_BACKEND = "wayland";
}
