# fan-control-rs

## References
- UI plans: https://github.com/Ax9D/pw-viz/blob/main/assets/demo.png
- Iced example on canvas: https://github.com/ua-kxie/circe



## Repo structure
- [hardware](./hardware/README.md): define an abstraction around the hardware.
- [data](./data/README.md): define structures used in the app (Node, Config), and there logic. Depend on [hardware](./hardware/README.md)
- [ui](./ui/README.md): implement the UI. Depend on [data](./data/README.md)
- the app: integrate all this crates into on executable


## Dependencies

#### Ubuntu
```
sudo apt install bison flex clang -y
```
#### Fedora
```
sudo dnf install lm_sensors-devel
```



steps

- create git submodule with this branch https://github.com/wiiznokes/libsensors/tree/pwm