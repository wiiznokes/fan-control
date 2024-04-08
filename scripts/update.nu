

def NEW_VERSION [] -> string {
    date now | format date "%-y.%m"
}

def NEW_VERSION_SEMVER_COMPAT [] -> string {
    date now | format date "%-y.%-m.0"
}

def NEW_DATE [] -> string {
    date now | format date "%Y-%m-%d"
}

def "main run" [] {
    
    let version = NEW_VERSION
    print $version
    let version = NEW_VERSION_SEMVER_COMPAT
    print $version
    let version = NEW_DATE
    print $version
}


def main [] {
}