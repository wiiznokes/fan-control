## run specific test:

```
clear && cargo test --package hardware test_time -- --nocapture
```

## Making a new release

0. get the version: `date +"%y.%m"`
1. change the version in VERSION file
2. change the version in Cargo.toml
3. make a pull request in https://github.com/flathub/io.github.wiiznokes.fan-control
4. launch the release workflow https://github.com/wiiznokes/fan-control/actions/workflows/release.yml
