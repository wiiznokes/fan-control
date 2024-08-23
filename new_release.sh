#!/bin/bash -ex

# cargo test --workspace --all-features

# cargo fmt --all --check --verbose

# cargo clippy --workspace --all-features

NEW_VERSION="$(date +"%-y.%-m.%-d")"

NEW_DATE="$(date +"%Y-%m-%d")"

sed -i '/<release /s/version="[^"]*"/version="'"$NEW_VERSION"'"/; /<release /s/date="[^"]*"/date="'"$NEW_DATE"'"/' "res/linux/metainfo.xml"

sed -i '/\[package.metadata.packager\]/,/^$/s/version = ".*"/version = "'"$NEW_VERSION"'"/' "Cargo.toml"

echo $NEW_VERSION >VERSION

changelog-gen generate --exclude-unidentified
changelog-gen release --version $NEW_VERSION

git add .
git commit -m "new release !log"
git push

gh release delete $NEW_VERSION -y || true
git tag -d $NEW_VERSION || true
git tag $NEW_VERSION || true
git push origin --tags || true

changelog-gen show >RELEASE_CHANGELOG.md

gh release create $NEW_VERSION --title $NEW_VERSION \
    --notes-file RELEASE_CHANGELOG.md --target $NEW_VERSION
