#!/bin/sh

set -e

OWNER="DragonTTV"
REPO="aether"

error() {
    printf "Error: %s\n" "$1" >&2
    exit 1
}

step() {
    printf "➜ %s\n" "$1"
}

success() {
    printf "✓ %s\n" "$1"
}

detect_platform() {
    step "Detecting operating system..."

    case "$(uname -s)" in
        Linux)
            PLATFORM="linux"
            ;;
        *)
            error "Unsupported operating system."
            ;;
    esac

    success "$PLATFORM detected"
}

detect_architecture() {
    step "Detecting architecture..."

    case "$(uname -m)" in
        x86_64|amd64)
            TARGET="x86_64"
            ;;
        aarch64|arm64)
            TARGET="aarch64"
            ;;
        *)
            error "Unsupported architecture."
            ;;
    esac

    success "$TARGET detected"
}

select_channel() {
    PRERELEASE=false
    CHANNEL_SPECIFIED=false

    while [ $# -gt 0 ]; do
        case "$1" in
            --pre)
                PRERELEASE=true
                CHANNEL_SPECIFIED=true
                ;;
            --stable)
                PRERELEASE=false
                CHANNEL_SPECIFIED=true
                ;;
        esac
        shift
    done

    if [ "$CHANNEL_SPECIFIED" = false ]; then
        printf "\n"
        printf "Select release channel:\n\n"
        printf "1) Stable (recommended)\n"
        printf "2) Pre-release\n\n"

        if [ -t 0 ]; then
            printf "Choice [1/2]: "
            read -r choice
        else
            printf "Choice [1/2]: " >/dev/tty
            read -r choice </dev/tty
        fi

        case "$choice" in
            2)
                PRERELEASE=true
                ;;
            *)
                PRERELEASE=false
                ;;
        esac
    fi

    if [ "$PRERELEASE" = true ]; then
        success "Selected Pre-release"
    else
        success "Selected Stable"
    fi
}

resolve_version() {
    API="https://api.github.com/repos/${OWNER}/${REPO}/releases"

    step "Resolving latest release..."

    if [ "$PRERELEASE" = true ]; then
        VERSION="$(
            curl -fsSL "$API" |
            sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' |
            head -n1
        )"
    else
        VERSION="$(
            curl -fsSL "$API/latest" |
            sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p'
        )"
    fi

    [ -n "$VERSION" ] || error "Failed to determine release version."

    success "$VERSION"
}

download_release() {
    ARCHIVE="aether-${PLATFORM}-${TARGET}.tar.gz"
    URL="https://github.com/${OWNER}/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

    step "Downloading Aether..."

    WORKDIR="$(mktemp -d)"

    curl -fsSL "$URL" -o "$WORKDIR/$ARCHIVE"

    success "Download complete"
}

extract_release() {
    step "Extracting archive..."

    tar -xzf "$WORKDIR/$ARCHIVE" -C "$WORKDIR"

    RELEASE_DIR="$WORKDIR/aether-${PLATFORM}-${TARGET}"

    success "Archive extracted"
}

run_installer() {
    step "Running installer..."

    chmod +x "$RELEASE_DIR/aether-setup"

    "$RELEASE_DIR/aether-setup"
}

cleanup() {
    step "Cleaning up..."

    rm -rf "$WORKDIR"

    success "Done"
}

main() {
    detect_platform
    detect_architecture
    select_channel "$@"
    resolve_version
    download_release
    extract_release
    run_installer
    cleanup
}

main "$@"