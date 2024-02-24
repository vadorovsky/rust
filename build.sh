#!/usr/bin/env bash

set -ex

TARGET_TRIPLE=${TARGET_TRIPLE:-}

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

if [[ -z "${TARGET_TRIPLE}" ]]; then
    unameOut="$(uname -s)-$(uname -m)"
    case "${unameOut}" in
        Linux-x86_64*)  TARGET_TRIPLE=x86_64-unknown-linux-gnu;;
        Linux-aarch64*) TARGET_TRIPLE=aarch64-unknown-linux-gnu;;
        Darwin-x86_64*) TARGET_TRIPLE=x86_64-apple-darwin;;
        Darwin-arm64*)  TARGET_TRIPLE=aarch64-apple-darwin;;
        MINGW*)         TARGET_TRIPLE=x86_64-pc-windows-msvc;;
        *)              TARGET_TRIPLE=x86_64-unknown-linux-gnu
    esac
fi

if [ "$1" == "--llvm" ]; then
    rm -f build/${TARGET_TRIPLE}/llvm/llvm-finished-building;
fi
./x.py build --stage 1 --target ${TARGET_TRIPLE},bpfel-unknown-unknown,sbf-solana-solana
