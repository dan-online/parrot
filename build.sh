#!/bin/bash
set -eu -o pipefail

#
# based on the output of
#   debtap -P src-tauri/target/release/bundle/deb/tauri-vite-example_0.1.0_amd64.deb
#
# see also https://github.com/tauri-apps/tauri/pull/4301
#

conf_json="src-tauri/tauri.conf.json"
bundle_dir="src-tauri/target/release/bundle"
build_dir="./build"

# extract package attributes
productName="parrot"
pkgver="$(jq -r '.package.version' < "$conf_json")"
pkgdesc="$(jq -r '.tauri.bundle.longDescription' < "$conf_json")"

# copy deb file locally
deb_path="$bundle_dir/deb/${productName}_${pkgver}_amd64.deb"
deb_basename="$(basename "$deb_path")"
mkdir -p "$build_dir"
cp -f "$deb_path" "$build_dir/$deb_basename"

# emit PKGBUILD
cat > "$build_dir/PKGBUILD" <<EOF
# Maintainer: DanCodes <dan@dancodes.online>
pkgname="parrot-bin"
pkgver="$pkgver"
pkgrel=1
pkgdesc="$pkgdesc"
arch=('x86_64')
url=""
license=('')
depends=('gtk3' 'webkit2gtk')
source=("$deb_basename")
sha512sums=("SKIP")

package(){
  tar -xz -f data.tar.gz -C "\${pkgdir}"
}
EOF