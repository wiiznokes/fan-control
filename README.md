# fan-control-rs


UI plans: https://github.com/Ax9D/pw-viz/blob/main/assets/demo.png

Iced example: https://github.com/ua-kxie/circe

Fonctionnement:

- programme qui crée/update le fichier hardware.toml + settings.toml
- programme qui verifie que une config est correcte.
- programme qui applique la config. Les settings et la config en question load au debut du programme. Si on veut stoper fan-control-rs, on peut le faire en tuant le programme. Dans les fonctions de drop, la remise a 0 des pwm devra être implementer.


Deep in




for Linux (Fedora)
```
sudo dnf install lm_sensors-devel
```

Ubuntu
- sudo apt install libsensors-dev