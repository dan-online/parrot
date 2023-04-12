#!/bin/bash
# Inspired by https://github.com/yochananmarqos/pkgbuilds/blob/master/parrot/PKGBUILD
set -eu -o pipefail

# variables
conf_json="./src-tauri/tauri.conf.json"
build_dir="./build"

# clean
rm -rf $build_dir
mkdir -p $build_dir

# extract package attributes
productName="parrot"
productTitle="Parrot"

# pkg variables
pkgver="$(jq -r '.package.version' < "$conf_json")"
pkgdesc="$(jq -r '.tauri.bundle.longDescription' < "$conf_json")"

# emit PKGBUILD
cat > "$build_dir/PKGBUILD" <<EOF
# Maintainer: DanCodes <dan@dancodes.online>
pkgname=$productName-git
pkgver="$pkgver"
pkgrel=1
pkgdesc="$pkgdesc"
arch=('x86_64')
url="https://github.com/dan-online/parrot"
license=('MIT')
depends=('gtk3' 'openssl' 'webkit2gtk')
makedepends=('cargo' 'node-gyp' 'yarn')
options=('!lto')
source=("git+https://github.com/dan-online/parrot.git")
sha256sums=('SKIP')

pkgver() {
  cd parrot
  git describe --long --tags | sed 's/\([^-]*-g\)/r\1/;s/-/./g'
}

prepare() {
  cd \${pkgname%-git}
  export YARN_CACHE_FOLDER="\$srcdir/yarn-cache"
  yarn install

  sed -i "s/\"productName\": \"$productTitle\"/\"productName\": \"$productTitle ($pkgver)\"/" "$conf_json"

  cd src-tauri
  export RUSTUP_TOOLCHAIN=stable
  cargo fetch --target "\$CARCH-unknown-linux-gnu"
}

build() {
  cd \${pkgname%-git}
  export YARN_CACHE_FOLDER="\$srcdir/yarn-cache"
  export RUSTUP_TOOLCHAIN=stable
  yarn tauri build
}

package() {
  cd \${pkgname%-git}

  pkgversion=\`sed -n 's/.*"version": "\(.*\)".*/\1/p' package.json\`
  pkgver_dash="\${pkgver//./-}"
  
  install -Dm755 "src-tauri/target/release/\${pkgname%-git}-\${pkgver_dash}" -t "\$pkgdir/usr/bin/"

  for i in 32x32 128x128 128x128@2x; do
    install -Dm644 src-tauri/icons/\${i}.png \
      "\$pkgdir/usr/share/icons/hicolor/\${i}/apps/\${pkgname%-git}.png"
  done
  install -Dm644 "src-tauri/target/release/bundle/deb/\${pkgname%-git}-\${pkgver_dash}_\${pkgversion}_amd64/data/usr/share/icons/hicolor/256x256@2/apps/\${pkgname%-git}-\${pkgver_dash}.png" -t \
    "\$pkgdir/usr/share/icons/hicolor/256x256@2/apps/\${pkgname%-git}.png"
  install -Dm644 src-tauri/icons/icon.png \
    "\$pkgdir/usr/share/icons/hicolor/512x512/apps/\${pkgname%-git}.png"

  install -Dm644 "src-tauri/target/release/bundle/deb/\${pkgname%-git}-\${pkgver_dash}_\${pkgversion}_amd64/data/usr/share/applications/\${pkgname%-git}-\${pkgver_dash}.desktop" -t \
    "\$pkgdir/usr/share/applications/"

  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/\${pkgname%-git}-\${pkgver_dash}/"
}
EOF
