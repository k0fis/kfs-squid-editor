#!/bin/bash
set -euo pipefail

REPO="k0fis/kfs-squid-editor"
INSTALL_DIR="/usr/local/bin"
BINARY="kfs-squid-editor"

# Detect platform
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

case "${OS}" in
  linux)  PLATFORM="linux" ;;
  darwin) PLATFORM="macos" ;;
  *) echo "Unsupported OS: ${OS}" >&2; exit 1 ;;
esac

case "${ARCH}" in
  x86_64|amd64)  ARCH_SUFFIX="amd64" ;;
  aarch64|arm64) ARCH_SUFFIX="arm64" ;;
  *) echo "Unsupported architecture: ${ARCH}" >&2; exit 1 ;;
esac

ARTIFACT="${BINARY}-${PLATFORM}-${ARCH_SUFFIX}"

# Get latest release tag
echo "Fetching latest release..."
TAG=$(curl -sI "https://github.com/${REPO}/releases/latest" | grep -i "^location:" | sed 's|.*/||' | tr -d '\r')

if [ -z "${TAG}" ]; then
  echo "Failed to determine latest release" >&2
  exit 1
fi

URL="https://github.com/${REPO}/releases/download/${TAG}/${ARTIFACT}"

echo "Downloading ${BINARY} ${TAG} (${PLATFORM}/${ARCH_SUFFIX})..."
curl -sL "${URL}" -o "/tmp/${BINARY}"
chmod +x "/tmp/${BINARY}"

# Install
if [ -w "${INSTALL_DIR}" ]; then
  mv "/tmp/${BINARY}" "${INSTALL_DIR}/${BINARY}"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo mv "/tmp/${BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

echo "Installed: $(${BINARY} --version)"
