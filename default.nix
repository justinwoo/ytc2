{ pkgs ? import <nixpkgs> {} }:

let
  binary = pkgs.rustPlatform.buildRustPackage rec {
    name = "ytc2-rs";
    version = "0.1.0";
    src = ./.;
    cargoSha256 = "0jacm96l1gw9nxwavqi1x4669cg6lzy9hr18zjpwlcyb3qkw9z7f";
  };

  requirements = import ./requirements.nix { inherit pkgs; };

in pkgs.runCommand "ytc2" {
  name = "ytc2";
  buildInputs = [
    pkgs.makeWrapper
  ];
} ''
    mkdir -p $out/bin
    install -D -m555 -t $out/bin ${binary}/bin/ytc2

    wrapProgram $out/bin/ytc2 \
      --prefix PICK_XSL : ${binary.src}/pick.xsl \
      --prefix PATH : ${pkgs.lib.makeBinPath [
        requirements.curl
        requirements.html-xml-utils
        requirements.libxslt
        requirements.jq
        requirements.youtube-dl
      ]}
  ''
