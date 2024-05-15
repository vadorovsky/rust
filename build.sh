#!/usr/bin/env bash

set -ex

if [ "$1" == "--help" ] || [ "$1" == "-h" ]; then
    echo "--llvm to rebuild llvm";
    exit;
fi

unameOut="$(uname -s)-$(uname -m)"
case "${unameOut}" in
    Linux*)
        LDD_VERSION=$(ldd --version 2>&1 | head -n 1 | tr '[:upper:]' '[:lower:]')
        if [[ "${LDD_VERSION}" == *"glibc"* ]]; then
            LIBC_SUFFIX="gnu"
        elif [[ "${LDD_VERSION}" == *"musl"* ]]; then
            LIBC_SUFFIX="musl"
        else
            echo "Could not determine the libc"
            exit 1
        fi

        if [[ "$(uname -m)" == "arm64" ]] || [[ "$(uname -m)" == "aarch64" ]]; then
            HOST_TRIPLE="aarch64-unknown-linux-${LIBC_SUFFIX}"
        else
            HOST_TRIPLE="x86_64-unknown-linux-${LIBC_SUFFIX}"
        fi
        ;;
    Darwin-x86_64*) HOST_TRIPLE=x86_64-apple-darwin;;
    Darwin-arm64*)  HOST_TRIPLE=aarch64-apple-darwin;;
    MINGW*)         HOST_TRIPLE=x86_64-pc-windows-msvc;;
    *)              HOST_TRIPLE=x86_64-unknown-linux-gnu
esac

if [ "$1" == "--llvm" ]; then
    rm -f build/${HOST_TRIPLE}/llvm/llvm-finished-building;
fi
./x.py build --stage 1 \
    --build ${HOST_TRIPLE} \
    --target ${HOST_TRIPLE},bpfel-unknown-unknown,sbf-solana-solana
