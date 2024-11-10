#!/bin/bash -xe

current_branch="$(git rev-parse --abbrev-ref HEAD)"
rm -rf io.github.wiiznokes.fan-control
git clone https://github.com/flathub/io.github.wiiznokes.fan-control
cd io.github.wiiznokes.fan-control
git branch -d new_release || true
git push origin --delete new_release || true
git checkout -b new_release
just branch=$current_branch
git add .
git commit -m "new release"
git push -u origin new_release
gh pr create --base master --head new_release --title "New release" --body ""
