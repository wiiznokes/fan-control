## Making a new release

- get the version: `date +"%-y.%-m.%-d"`
- update release section of [metadata file](./res/linux/metainfo.xml)
- change the version in [Cargo.toml](./Cargo.toml)
- change the version in [VERSION](./VERSION)


- new release in the [CHANGELOG](./CHANGELOG.md)
- make a pull request in [here](https://github.com/flathub/io.github.wiiznokes.fan-control)
- launch the release workflow [here](https://github.com/wiiznokes/fan-control/actions/workflows/release.yml)

## run specific test:

```
clear && cargo test --package hardware test_time -- --nocapture
```
