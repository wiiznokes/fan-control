# fan-control-rs


API:

- set_timmer(delay: u8)
- set_config(file_name: Option(String))
- start()
- stop()
- status() -> Status

- controls_values() -> (String, u8)
- fans_values() -> (String, u8)
- temps_values() -> (String, u8)

- compute_behavior(behavior: Behavior) -> Option(u8)



TODO:
- appeler du code C# en Rust (pas obligé)
- appeler les fonction de lm sensor en Rust
- appeler a travers le local host des fonctions Rust, notament compute_behavior, qui prend en parametre une behavior entier.

Possibilité pour le troisième point
- utiliser GRPC (mais ducoup, on est obligé d'utiliser protobuf alors que la config utilise serde)
- utiliser une requete transcoder en JSON ou TOML.
- regarder comment s'en sort Open RGB, qui utilise aussi une API sur le reseau.