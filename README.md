# fan-control-rs

UI plans: https://github.com/Ax9D/pw-viz/blob/main/assets/demo.png
Iced example: https://github.com/ua-kxie/circe




crates:
- ui (define the UI), <- config
- config (define the configs files, settings, hardware, ser/de),
- hardware (define an abstraction around the hardware), <- config


Arch:

Each node/item/vertice have an unique ID, stored in a global hashmap.
hardware is responsible to implement an interface for each hardware items


trait HardwareSpec {

    fn hardware_type() -> HardwareType;

    fn hardware_id -> Option<u32>;


}

trait Hardware {
    generate_temp_id<T: HardwareSpec>(&mut item: T);

    value<T: HardwareSpec>(&item: T) -> Option<i32>;

    set_value<T: HardwareSpec>(&item: T, value: i32);

}






Fedora
```
sudo dnf install lm_sensors-devel
```

Ubuntu
```
sudo apt install libsensors-dev
```

clean code
```
cargo clippy --all --fix --allow-dirty --allow-staged
cargo fmt --all
```