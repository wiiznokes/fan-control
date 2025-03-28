## run specific test:

```
clear && cargo test --package hardware test_time -- --nocapture
```

change before release

- version in [justfile](./justfile)
- version in [metainfo](./res/linux/metainfo.xml)
- version in [Cargo.toml](./Cargo.toml)
- release in [CHANGELOG.md](./CHANGELOG.md)