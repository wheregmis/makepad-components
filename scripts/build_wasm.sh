#!/usr/bin/env bash

set -euo pipefail

usage() {
    cat <<'EOF'
Usage: scripts/build_wasm.sh [-p package] [--profile profile] [--release] [--bindgen] [--no-threads] [--no-brotli] [extra cargo-makepad wasm flags]

Builds a Makepad wasm app using cargo-makepad with --wasm-opt and --strip enabled.
EOF
}

APP_PACKAGE="makepad-gallery"
PROFILE="small"
MODE_FLAGS=()
EXTRA_FLAGS=()
HAS_RELEASE=0
HAS_PROFILE=0
ENABLE_BROTLI=1

patch_bindgen_js() {
    local bindgen_file="$1"

    if [[ ! -f "${bindgen_file}" ]]; then
        return 0
    fi

    perl -0pi -e '
        s/const imports=__wbg_get_imports\(memory\);/const imports=__wbg_get_imports(memory);imports.env=env;/g;
        s/const imports = __wbg_get_imports\(memory\);/const imports = __wbg_get_imports(memory); imports.env = env;/g;
        s/imports=__wbg_get_imports\(\);/imports=__wbg_get_imports(); imports.env=env;/g;
        s/imports = __wbg_get_imports\(\);/imports = __wbg_get_imports(); imports.env = env;/g;
    ' "${bindgen_file}"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        -p|--package)
            APP_PACKAGE="${2:?missing package name}"
            shift 2
            ;;
        --profile)
            PROFILE="${2:?missing profile name}"
            HAS_PROFILE=1
            shift 2
            ;;
        --profile=*)
            PROFILE="${1#*=}"
            if [[ -z "${PROFILE}" ]]; then
                echo "error: missing profile name" >&2
                exit 1
            fi
            HAS_PROFILE=1
            shift
            ;;
        --release|--bindgen|--no-threads)
            if [[ "$1" == "--release" ]]; then
                HAS_RELEASE=1
            fi
            MODE_FLAGS+=("$1")
            shift
            ;;
        --no-brotli)
            ENABLE_BROTLI=0
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

if [[ ${HAS_RELEASE} -eq 1 && ${HAS_PROFILE} -eq 1 ]]; then
    echo "error: --release and --profile cannot be used together" >&2
    exit 1
fi

CMD=(cargo makepad wasm --wasm-opt --strip)

if [[ ${ENABLE_BROTLI} -eq 1 ]]; then
    CMD+=(--brotli)
fi

CMD+=("${MODE_FLAGS[@]}")
CMD+=(build -p "${APP_PACKAGE}")

if [[ ${HAS_RELEASE} -eq 0 ]]; then
    CMD+=("--profile=${PROFILE}")
fi

CMD+=("${EXTRA_FLAGS[@]}")

"${CMD[@]}"

OUTPUT_PROFILE="${PROFILE}"
if [[ ${HAS_RELEASE} -eq 1 ]]; then
    OUTPUT_PROFILE="release"
fi

patch_bindgen_js "target/makepad-wasm-app/${OUTPUT_PROFILE}/${APP_PACKAGE}/bindgen.js"
