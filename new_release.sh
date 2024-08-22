#!/bin/bash

NEW_VERSION="$(date +"%-y.%-m.%-d")"

NEW_DATE="$(date +"%Y-%m-%d")"

sed -i '/<release /s/version="[^"]*"/version="'"$NEW_VERSION"'"/; /<release /s/date="[^"]*"/date="'"$NEW_DATE"'"/' "res/linux/metainfo.xml"

sed -i '/\[package.metadata.packager\]/,/^$/s/version = ".*"/version = "'"$NEW_VERSION"'"/' "Cargo.toml"

echo $NEW_VERSION > VERSION