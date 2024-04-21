use std::vec;

use hardware::{Hardware, Value};
use serde::{Deserialize, Deserializer, Serialize};

use crate::{
    app_graph::Nodes,
    id::IdGenerator,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

use super::utils::affine::Affine;

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Coord {
    pub temp: u8,
    pub percent: u8,
}

impl TryFrom<(&str, &str)> for Coord {
    type Error = Box<dyn std::error::Error>;

    fn try_from((temp, percent): (&str, &str)) -> Result<Self, Self::Error> {
        let temp = temp.parse::<u8>()?;

        let percent = percent.parse::<u8>()?;

        if percent > 100 {
            return Err("Percent > 100".into());
        }

        Ok(Coord { temp, percent })
    }
}

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        self.temp == other.temp
    }
}

impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Coord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.temp.cmp(&other.temp)
    }
}

impl Coord {
    pub fn exact_same(&self, other: &Self) -> bool {
        self.percent == other.percent && self.temp == other.temp
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub name: String,
    #[serde(rename = "coord")]
    pub coords: Coords,
    pub input: Option<String>, // Temp or CustomTemp
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            name: Default::default(),
            coords: Coords(vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 70,
                    percent: 100,
                },
            ]),
            input: Default::default(),
        }
    }
}

impl ToNode for Graph {
    fn to_node(self, id_generator: &mut IdGenerator, nodes: &Nodes, _hardware: &Hardware) -> Node {
        Node::new(id_generator, NodeType::Graph(self), nodes)
    }
}

impl IsValid for Graph {
    // todo: we can filter wrong Coord when launching
    // and only autorize good coord to be added so we don't need this
    // kind on verification
    // so only check if !empty
    fn is_valid(&self) -> bool {
        #[derive(PartialEq)]
        enum DupState {
            Init,
            Prev { temp: u8 },
            DuplicateFound,
        }

        self.input.is_some()
            && !self.coords.0.is_empty()
            && self
                .coords
                .0
                .iter()
                .fold(DupState::Init, |prev, coord| match prev {
                    DupState::Init => DupState::Prev { temp: coord.temp },
                    DupState::Prev { temp } => {
                        if temp == coord.temp {
                            DupState::DuplicateFound
                        } else {
                            DupState::Prev { temp: coord.temp }
                        }
                    }
                    DupState::DuplicateFound => DupState::DuplicateFound,
                })
                != DupState::DuplicateFound
            && !self.coords.0.iter().any(|coord| coord.percent > 100)
    }
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct Coords(pub Vec<Coord>);

impl<'de> serde::Deserialize<'de> for Coords {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let mut s: Vec<Coord> = Vec::deserialize(d)?;

        s.sort();

        Ok(Coords(s))
    }
}

impl Graph {
    pub fn get_value(&self, value: Value) -> Result<Value, UpdateError> {
        let dummy_coord = Coord {
            temp: value as u8,
            percent: 0,
        };

        let res = match self.coords.0.binary_search(&dummy_coord) {
            Ok(index) => self.coords.0[index].percent as Value,
            Err(index) => {
                if index == 0 {
                    self.coords.0[index].percent as Value
                } else if index == self.coords.0.len() {
                    self.coords.0[index - 1].percent as Value
                } else {
                    let coord1 = &self.coords.0[index - 1];
                    let coord2 = &self.coords.0[index];

                    Affine {
                        xa: coord1.temp.into(),
                        ya: coord1.percent.into(),
                        xb: coord2.temp.into(),
                        yb: coord2.percent.into(),
                    }
                    .calcule(value) as Value
                }
            }
        };

        Ok(res)
    }
}

#[cfg(test)]
mod test {
    use crate::config::graph::{Coord, Coords};

    #[test]
    fn test() {
        let coord1 = Coord {
            temp: 10,
            percent: 10,
        };

        let coord2 = Coord {
            temp: 20,
            percent: 20,
        };

        let coord3 = Coord {
            temp: 30,
            percent: 30,
        };

        let coord4 = Coord {
            temp: 40,
            percent: 40,
        };

        let coords = Coords(vec![coord1, coord2, coord3, coord4]);

        let dummy_coord = Coord {
            temp: 50,
            percent: 0,
        };

        let res = coords.0.binary_search(&dummy_coord);

        match res {
            Ok(index) => {
                println!("use {}", index);
            }
            Err(index) => {
                if index == 0 {
                    println!("use {}", index);
                } else if index == coords.0.len() {
                    println!("use {}", index - 1);
                } else {
                    println!("use {} and {}", index - 1, index);
                }
            }
        }
        dbg!(&res);
    }
}
