VERSION=$1

# If no version, exit
if [ -z "$VERSION" ]; then
  echo "No version specified"
  exit 1
fi

# If version is not a valid semver, exit
if ! echo "$VERSION" | grep -qE "^[0-9]+\.[0-9]+\.[0-9]+$"; then
  echo "Version is not a valid semver"
  exit 1
fi

# bump package.json
jq ".version = \"$VERSION\"" package.json > package.json.tmp
mv package.json.tmp package.json

# bump tauri.conf.json
jq ".package.version = \"$VERSION\"" src-tauri/tauri.conf.json > src-tauri/tauri.conf.json.tmp
mv src-tauri/tauri.conf.json.tmp src-tauri/tauri.conf.json

# bump Cargo.toml
sed -i "s/^version = \".*\"/version = \"$VERSION\"/" src-tauri/Cargo.toml