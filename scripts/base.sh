#!/bin/bash
set -eu -o pipefail

# variables
conf_json="./src-tauri/tauri.conf.json"
build_dir="./build"
commit_id="$(git rev-parse --short HEAD)"
full_commit_id="$(git rev-parse HEAD)"

# clean
rm -rf $build_dir
mkdir -p $build_dir

# get repo at commit_id as zip
source_url="https://github.com/dan-online/parrot/archive/$full_commit_id.tar.gz"
sha256sums="$(curl -sSL "$source_url" | sha256sum | cut -d' ' -f1)"

# extract package attributes
productName="parrot"
productTitle="Parrot"
pkgver="$(jq -r '.package.version' < "$conf_json")"
pkgdesc="$(jq -r '.tauri.bundle.longDescription' < "$conf_json")"

# emit PKGBUILD
cat > "$build_dir/PKGBUILD" <<EOF
# Maintainer: DanCodes <dan@dancodes.online>
pkgname=$productName-git
pkgver="$pkgver.$commit_id"
pkgrel=1
pkgdesc="$pkgdesc"
arch=('x86_64')
url="https://github.com/dan-online/parrot"
license=('MIT')
depends=('gtk3' 'openssl' 'webkit2gtk')
makedepends=('cargo' 'node-gyp' 'yarn')
options=('!lto')
source=("\$pkgname-\$pkgver.tar.gz::$source_url")
sha256sums=('${sha256sums}')

prepare() {
  cd "$productName-$full_commit_id"
  export YARN_CACHE_FOLDER="\$srcdir/yarn-cache"
  yarn install

  sed -i "s/\"productName\": \"$productTitle\"/\"productName\": \"$productTitle ($commit_id)\"/" "$conf_json"

  cd src-tauri
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --target "\$CARCH-unknown-linux-gnu"
}

build() {
  cd "$productName-$full_commit_id"
  export YARN_CACHE_FOLDER="\$srcdir/yarn-cache"
  export RUSTUP_TOOLCHAIN=stable
  yarn tauri build
}

package() {
  cd "$productName-$full_commit_id"
  
  install -Dm755 "src-tauri/target/release/$productName-$commit_id" -t "\$pkgdir/usr/bin/"

  for i in 32x32 128x128 128x128@2x; do
    install -Dm644 src-tauri/icons/\${i}.png \
      "\$pkgdir/usr/share/icons/hicolor/\${i}/apps/$productName.png"
  done
  install -Dm644 "src-tauri/target/release/bundle/deb/${productName}-${commit_id}_${pkgver}_amd64/data/usr/share/icons/hicolor/256x256@2/apps/${productName}-${commit_id}.png" -t \
    "\$pkgdir/usr/share/icons/hicolor/256x256@2/apps/$productName.png"
  install -Dm644 src-tauri/icons/icon.png \
    "\$pkgdir/usr/share/icons/hicolor/512x512/apps/$productName.png"

  install -Dm644 "src-tauri/target/release/bundle/deb/${productName}-${commit_id}_${pkgver}_amd64/data/usr/share/applications/${productName}-${commit_id}.desktop" -t \
    "\$pkgdir/usr/share/applications/"

  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/${productName}-${commit_id}/"
}
EOF