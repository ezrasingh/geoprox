{ pkgs ? import <nixpkgs> { } }:

let
  inherit (pkgs) stdenv;
  NPM_CONFIG_PREFIX = toString ./npm_config_prefix;

in pkgs.mkShell {
  name = "openapi-generator-environment";
  buildInputs = with pkgs; [ nodejs_18 yarn jre just jrsonnet ];

  inherit NPM_CONFIG_PREFIX;

  shellHook = ''
    export PATH="${NPM_CONFIG_PREFIX}/bin:$PATH"
    alias jsonnet="jrsonnet"
    yarn install
  '';
}
