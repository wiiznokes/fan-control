

def NEW_VERSION [] -> string {
    date now | format date "%Y.%m"
}

def NEW_VERSION_SEMVER_COMPAT [] -> string {
    date now | format date "%Y.%-m.0"
}

def NEW_DATE [] -> string {
    date now | format date "%Y-%m-%d"
}


def "main change_version" [] {
    
    let version_compat = NEW_VERSION_SEMVER_COMPAT
    open Cargo.toml --raw | decode utf-8 | 
        str replace -r '\[package\.metadata\.packager\]\n\s*version\s*=\s*".*"' $"[package.metadata.packager]\nversion = \"($version_compat)\"" | 
        save Cargo.toml -f

    let meta_file = "res/linux/metainfo.xml"
    let version = NEW_VERSION
    let date = NEW_DATE

    open $meta_file --raw | decode utf-8 |
        str replace -r '<release version=".*" date=".*">' $"<release version=\"($version)\" date=\"($date)\">" |
        save $meta_file -f

}

def "main changen" [] {
    
    let version = NEW_VERSION
    exec changen generate --repo "wiiznokes/fan-control"
    exec changen release --version $version

}

def "main gh_release" [] {


    let version = NEW_VERSION
    
    exec gh release delete $version -y
    exec git tag -d $version
    exec git push origin --delete $version
    exec git tag $version
    exec git push origin $version

    exec changelog-gen show >RELEASE_CHANGELOG.md

    let SHA = (exec git rev-parse $version).stdout

    exec gh release create $version --title $version --notes-file RELEASE_CHANGELOG.md --target $SHA


}

def "main pass_test" [] {

    exec cargo test --workspace --all-features
    exec cargo fmt --all --check --verbose
    exec cargo clippy --workspace --all-features
}


def main [] {
}

