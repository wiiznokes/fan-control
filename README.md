# fan-control-rs



Fonctionnement:

- programme qui crée/update le fichier hardware.toml + settings.toml
- programme qui verifie que une config est correcte.
- programme qui applique la config. Les settings et la config en question load au debut du programme. Si on veut stoper fan-control-rs, on peut le faire en tuant le programme. Dans les fonctions de drop, la remise a 0 des pwm devra être implementer.


Deep in




TODO:
- appeler du code C# en Rust (pas obligé)
- appeler les fonction de lm sensor en Rust
- appeler a travers le local host des fonctions Rust, notament compute_behavior, qui prend en parametre une behavior entier.

Possibilité pour le troisième point
- utiliser GRPC (mais ducoup, on est obligé d'utiliser protobuf alors que la config utilise serde)
- utiliser une requete transcoder en JSON ou TOML.
- regarder comment s'en sort Open RGB, qui utilise aussi une API sur le reseau.




for Linux (Fedora)
```
sudo dnf install lm_sensors-devel
```