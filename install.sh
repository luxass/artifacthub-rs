#!/bin/sh

set -eu

repo="luxass/artifacthub-mcp"
install_dir="${ARTIFACTHUB_MCP_INSTALL_DIR:-${HOME}/.local/bin}"
version="${ARTIFACTHUB_MCP_VERSION:-latest}"

need_cmd() {
    if ! command -v "$1" >/dev/null 2>&1; then
        echo "error: missing required command: $1" >&2
        exit 1
    fi
}

detect_os() {
    case "$(uname -s)" in
        Darwin) echo "macos" ;;
        Linux) echo "linux" ;;
        *)
            echo "error: unsupported operating system: $(uname -s)" >&2
            exit 1
            ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        arm64|aarch64) echo "aarch64" ;;
        x86_64|amd64) echo "x86_64" ;;
        *)
            echo "error: unsupported architecture: $(uname -m)" >&2
            exit 1
            ;;
    esac
}

download() {
    url="$1"
    output="$2"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url" -o "$output"
        return
    fi

    if command -v wget >/dev/null 2>&1; then
        wget -qO "$output" "$url"
        return
    fi

    echo "error: either curl or wget is required" >&2
    exit 1
}

download_to_stdout() {
    url="$1"

    if command -v curl >/dev/null 2>&1; then
        curl -fsSL "$url"
        return
    fi

    if command -v wget >/dev/null 2>&1; then
        wget -qO- "$url"
        return
    fi

    echo "error: either curl or wget is required" >&2
    exit 1
}

install_binary() {
    src="$1"
    dst="$2"

    mkdir -p "$dst"

    if command -v install >/dev/null 2>&1; then
        install "$src" "$dst/artifacthub-mcp"
    else
        cp "$src" "$dst/artifacthub-mcp"
        chmod +x "$dst/artifacthub-mcp"
    fi
}

resolve_latest_version() {
    api_url="https://api.github.com/repos/${repo}/releases/latest"
    download_to_stdout "$api_url" | sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\(v[^"]*\)".*/\1/p' | head -n 1
}

install_via_brew() {
    if command -v brew >/dev/null 2>&1; then
        echo "detected Homebrew, installing via brew..."
        brew install luxass/homebrew-tap/artifacthub-mcp
        return 0
    fi
    return 1
}

install_via_release() {
    os="$1"
    arch="$2"
    ver="$3"

    target="${arch}-unknown-linux-gnu"
    if [ "$os" = "macos" ]; then
        target="${arch}-apple-darwin"
    fi

    archive_url="https://github.com/${repo}/releases/download/v${ver}/artifacthub-mcp-${ver}-${target}.tar.gz"

    tmp_dir="$(mktemp -d)"
    trap 'rm -rf "$tmp_dir"' EXIT INT TERM

    archive_path="${tmp_dir}/artifacthub-mcp.tar.gz"
    extract_dir="${tmp_dir}/extract"
    mkdir -p "$extract_dir"

    download "$archive_url" "$archive_path"

    tar -xzf "$archive_path" -C "$extract_dir"

    binary_path="$(find "$extract_dir" -type f -name artifacthub-mcp | head -n 1)"

    if [ -z "$binary_path" ]; then
        echo "error: artifacthub-mcp binary not found in release archive" >&2
        exit 1
    fi

    install_binary "$binary_path" "$install_dir"
    echo "installed artifacthub-mcp to ${install_dir}/artifacthub-mcp"
}

need_cmd uname
need_cmd tar
need_cmd mktemp

os="$(detect_os)"
arch="$(detect_arch)"

if install_via_brew; then
    exit 0
fi

if [ "$version" = "latest" ]; then
    resolved_version="$(resolve_latest_version)"

    if [ -z "$resolved_version" ]; then
        echo "error: failed to resolve the latest release version" >&2
        exit 1
    fi

    version="${resolved_version#v}"
fi

install_via_release "$os" "$arch" "$version"
