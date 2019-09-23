{ pkgs ? import <nixpkgs> {} }:

let
  requirements = import ./requirements.nix { inherit pkgs; };
  dynamic-linker = pkgs.stdenv.cc.bintools.dynamicLinker;
  libPath = pkgs.lib.makeLibraryPath [ pkgs.glibc ];
in
pkgs.runCommand "ytc2" {
  name = "ytc2";

  src = ./output;

  buildInputs = [
    pkgs.makeWrapper
  ];
} ''
  mkdir -p $out/bin
  install -D -m555 -t $out/bin $src/ytc2

  YTC2=$out/bin/ytc2

  chmod +w $YTC2
  patchelf --interpreter ${dynamic-linker} --set-rpath ${libPath} $YTC2

  wrapProgram $YTC2 \
    --prefix PICK_XSL : $src/pick.xsl \
    --prefix PATH : ${pkgs.lib.makeBinPath (builtins.attrValues requirements)}
''
