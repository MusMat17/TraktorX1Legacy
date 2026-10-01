#!/usr/bin/env bash
set -euo pipefail

BUILD_TARGET="${1:-x86_64-apple-darwin}"
MIN_MACOS="${MACOSX_DEPLOYMENT_TARGET:-12.0}"
APP_NAME="TraktorX1Legacy"
BINARY_NAME="traktor-x1-legacy"
VERSION="$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -1)"
BUNDLE_ROOT="target/${BUILD_TARGET}/release/bundle/osx"
APP_BUNDLE_PATH="${BUNDLE_ROOT}/${APP_NAME}.app"
CONTENTS="${APP_BUNDLE_PATH}/Contents"
MACOS_DIR="${CONTENTS}/MacOS"
RESOURCES_DIR="${CONTENTS}/Resources"
BIN_PATH="${MACOS_DIR}/${BINARY_NAME}"

export MACOSX_DEPLOYMENT_TARGET="${MIN_MACOS}"

echo "Building ${APP_NAME} ${VERSION} for ${BUILD_TARGET}, macOS ${MIN_MACOS}+"
cargo build --release --target="${BUILD_TARGET}"

rm -rf "${APP_BUNDLE_PATH}"
mkdir -p "${MACOS_DIR}" "${RESOURCES_DIR}"
cp "target/${BUILD_TARGET}/release/${BINARY_NAME}" "${BIN_PATH}"
cp board.yml "${RESOURCES_DIR}/board.yml"

cat > "${CONTENTS}/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>en</string>
  <key>CFBundleExecutable</key>
  <string>${BINARY_NAME}</string>
  <key>CFBundleIdentifier</key>
  <string>io.github.musmat17.traktorx1legacy</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>${APP_NAME}</string>
  <key>CFBundleDisplayName</key>
  <string>${APP_NAME}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>${VERSION}</string>
  <key>CFBundleVersion</key>
  <string>${VERSION}</string>
  <key>LSMinimumSystemVersion</key>
  <string>${MIN_MACOS}</string>
  <key>LSUIElement</key>
  <true/>
</dict>
</plist>
PLIST

LIBUSB_DYLIB_PATH="$(otool -L "${BIN_PATH}" | awk '/libusb/{print $1; exit}')"
if [[ -z "${LIBUSB_DYLIB_PATH}" || ! -f "${LIBUSB_DYLIB_PATH}" ]]; then
  echo "Could not locate the libusb dylib linked by ${BIN_PATH}" >&2
  otool -L "${BIN_PATH}" >&2
  exit 1
fi

LIBUSB_DYLIB_NAME="$(basename "${LIBUSB_DYLIB_PATH}")"
cp "${LIBUSB_DYLIB_PATH}" "${MACOS_DIR}/${LIBUSB_DYLIB_NAME}"
install_name_tool -change   "${LIBUSB_DYLIB_PATH}"   "@executable_path/${LIBUSB_DYLIB_NAME}"   "${BIN_PATH}"

codesign --force --sign - "${MACOS_DIR}/${LIBUSB_DYLIB_NAME}"
codesign --force --deep --sign - "${APP_BUNDLE_PATH}"

mkdir -p dist
ZIP_NAME="dist/TraktorX1Legacy_${BUILD_TARGET}_macos${MIN_MACOS}+.zip"
rm -f "${ZIP_NAME}"
ditto -c -k --sequesterRsrc --keepParent "${APP_BUNDLE_PATH}" "${ZIP_NAME}"

echo "Created ${ZIP_NAME}"
