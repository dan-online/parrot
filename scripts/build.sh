#!/bin/bash
set -eu -o pipefail

# variables
conf_json="./src-tauri/tauri.conf.json"
build_dir="./build"

# clean
rm -rf $build_dir
mkdir -p $build_dir

# extract package attributes
productName="parrot"
pkgver="$(jq -r '.package.version' < "$conf_json")"
pkgdesc="$(jq -r '.tauri.bundle.longDescription' < "$conf_json")"

# copy deb file locally
deb_path="$(find -name '*.deb' | head -n 1)"
deb_basename="$(basename "$deb_path")"
cp -f "$deb_path" "$build_dir/$deb_basename"

# emit PKGBUILD
cat > "$build_dir/PKGBUILD" <<EOF
# Maintainer: DanCodes <dan@dancodes.online>
pkgname="parrot-bin"
pkgver="$pkgver"
pkgdesc="$pkgdesc"
arch=('x86_64')
url="https://github.com/dan-online/parrot"
license=('MIT')
depends=('gtk3' 'webkit2gtk')
source=("")
sha512sums=("SKIP")

package(){
  tar -xz -f data.tar.gz -C "\${pkgdir}"
}
EOF