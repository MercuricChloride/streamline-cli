{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell { packages = with pkgs; [ protobuf rustup pkg-config openssl ]; }
