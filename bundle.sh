#!/usr/bin/env bash
set -euo pipefail

BUILD_TARGET="${1:-x86_64-apple-darwin}"
MIN_MACOS="${MACOSX_DEPLOYMENT_TARGET:-12.0}"
APP_NAME="TraktorX1Legacy"
BINARY_NAME="traktor-x1-legacy"
APP_BUNDLE_PATH="target/${BUILD_TARGET}/release/bundle/osx/${APP_NAME}.app"
BIN_PATH="${APP_BUNDLE_PATH}/Contents/MacOS/${BINARY_NAME}"

export MACOSX_DEPLOYMENT_TARGET="${MIN_MACOS}"

echo "Building ${APP_NAME} for ${BUILD_TARGET}, deployment target macOS ${MIN_MACOS}"

rustup target add "${BUILD_TARGET}"

if ! command -v cargo-bundle >/dev/null 2>&1; then
  cargo install cargo-bundle --locked
fi

cargo bundle --release --target="${BUILD_TARGET}"

if [[ ! -f "${BIN_PATH}" ]]; then
  echo "Bundle binary not found: ${BIN_PATH}" >&2
  exit 1
fi

LIBUSB_DYLIB_PATH="$(otool -L "${BIN_PATH}" | awk '/libusb/{print $1; exit}')"
if [[ -n "${LIBUSB_DYLIB_PATH}" && -f "${LIBUSB_DYLIB_PATH}" ]]; then
  LIBUSB_DYLIB_NAME="$(basename "${LIBUSB_DYLIB_PATH}")"
  cp "${LIBUSB_DYLIB_PATH}" "${APP_BUNDLE_PATH}/Contents/MacOS/"
  install_name_tool -change "${LIBUSB_DYLIB_PATH}" "@executable_path/${LIBUSB_DYLIB_NAME}" "${BIN_PATH}"
fi

codesign --force --deep --sign - "${APP_BUNDLE_PATH}"

mkdir -p dist
ZIP_NAME="dist/TraktorX1Legacy_${BUILD_TARGET}_macos${MIN_MACOS}+.zip"
rm -f "${ZIP_NAME}"
ditto -c -k --sequesterRsrc --keepParent "${APP_BUNDLE_PATH}" "${ZIP_NAME}"

echo "Created ${ZIP_NAME}"
