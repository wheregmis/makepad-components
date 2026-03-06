#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/build_wasm.sh [-p package] [--profile profile] [--release] [--bindgen] [--no-threads]

Builds a Makepad wasm app, runs wasm-opt -Oz on the produced .wasm, and refreshes the Brotli asset.
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

install_hint() {
    local tool="$1"
    case "$(uname -s)" in
        Darwin)
            echo "Install with: brew install binaryen brotli" >&2
            ;;
        Linux)
            echo "Install with your package manager, for example: sudo apt-get install -y binaryen brotli" >&2
            ;;
        *)
            echo "Install ${tool} and ensure it is on PATH." >&2
            ;;
    esac
}

if ! command -v wasm-opt >/dev/null 2>&1; then
    echo "error: wasm-opt not found. Install Binaryen first." >&2
    install_hint "wasm-opt"
    exit 1
fi

if ! command -v brotli >/dev/null 2>&1; then
    echo "error: brotli not found. Install the brotli CLI first." >&2
    install_hint "brotli"
    exit 1
fi

cargo makepad wasm --strip --brotli build -p "${APP_PACKAGE}" --profile="${PROFILE}" "${MODE_FLAGS[@]}" "${EXTRA_FLAGS[@]}"

APP_DIR="target/makepad-wasm-app/${PROFILE}/${APP_PACKAGE}"
WASM_PATH="${APP_DIR}/${APP_PACKAGE}.wasm"
TMP_PATH="${WASM_PATH}.opt"

if [[ ! -f "${WASM_PATH}" ]]; then
    echo "error: expected wasm artifact not found at ${WASM_PATH}" >&2
    exit 1
fi

wasm-opt -Oz --all-features "${WASM_PATH}" -o "${TMP_PATH}"
mv "${TMP_PATH}" "${WASM_PATH}"

rm -f "${WASM_PATH}.br"
brotli --force --best "${WASM_PATH}"
