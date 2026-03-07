#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/build_wasm.sh [-p package] [--profile profile] [--release] [--bindgen] [--no-threads]

Builds a Makepad wasm app using cargo-makepad with --strip and --brotli enabled.
EOF
}

APP_PACKAGE="makepad-example-component-gallery"
PROFILE="small"
MODE_FLAGS=()
EXTRA_FLAGS=()

while [[ $# -gt 0 ]]; do
    case "$1" in
        -p|--package)
            APP_PACKAGE="${2:?missing package name}"
            shift 2
            ;;
        --profile)
            PROFILE="${2:?missing profile name}"
            shift 2
            ;;
        --release|--bindgen|--no-threads)
            MODE_FLAGS+=("$1")
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            EXTRA_FLAGS+=("$1")
            shift
            ;;
    esac
done

cargo makepad wasm --wasm-opt --strip --split --brotli --no-threads build -p "${APP_PACKAGE}" --profile=small
