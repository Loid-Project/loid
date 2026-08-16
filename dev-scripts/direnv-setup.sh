#!/bin/bash/env bash

curl --proto '=https' --tlsv1.2 -L https://nixos.org/nix/install | sh -s -- --daemon

nix-env -iA nixpkgs.direnv

direnv allow
