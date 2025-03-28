#!/bin/bash -xe

NEW_VERSION="2025.3.0"

gh release delete $NEW_VERSION -y || true
git tag -d $NEW_VERSION || true
git push origin --delete $NEW_VERSION || true
git tag $NEW_VERSION
git push origin $NEW_VERSION

changelog-gen show >RELEASE_CHANGELOG.md

SHA="$(git rev-parse $NEW_VERSION)"

gh release create $NEW_VERSION --title $NEW_VERSION \
    --notes-file RELEASE_CHANGELOG.md --target $SHA
