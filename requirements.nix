{ pkgs ? import <nixpkgs> {} }:

{
  inherit (pkgs)
  curl
  html-xml-utils
  libxslt
  jq
  sqlite
  youtube-dl;
}
