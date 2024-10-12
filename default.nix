{ pkgs }: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname              = manifest.name;
    version            = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src                = pkgs.lib.cleanSource ./.;
    
    nativeBuildInputs = with pkgs; [
      pkg-config
    ];

    buildInputs = with pkgs; [
      zstd
    ] ++ pkgs.lib.optionals stdenv.isLinux (with pkgs; [
      alsa-lib
      alsa-lib.dev
      libxkbcommon
      udev
      udev.dev
      vulkan-loader
      wayland
      libxkbcommon
      xorg.libX11
      xorg.libXcursor
      xorg.libXi
      xorg.libXrandr
    ]) ++ pkgs.lib.optionals stdenv.isDarwin (with pkgs; [
      darwin.apple_sdk_11_0.frameworks.Cocoa
    ]);

    ZSTD_SYS_USE_PKG_CONFIG = true;
    LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    PKG_CONFIG_PATH = pkgs.lib.makeLibraryPath buildInputs;
  }
