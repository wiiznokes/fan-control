## run specific test:

```
clear && cargo test --package hardware test_time -- --nocapture
```

## Making a new release

- get the version: `date +"%y.%m"`
- new release in the [CHANGELOG](./CHANGELOG.md)
- change the version in [VERSION](./VERSION)
- change the version in [Cargo.toml](./Cargo.toml)
- make a pull request in [here](https://github.com/flathub/io.github.wiiznokes.fan-control)
- launch the release workflow [here](https://github.com/wiiznokes/fan-control/actions/workflows/release.yml) 
