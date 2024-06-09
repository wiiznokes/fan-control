use std::{collections::HashSet, hash::Hash, vec};

use hardware::{Hardware, Value};
use serde::{Deserialize, Serialize};

use crate::{
    app_graph::AppGraph,
    node::{IsValid, Node, NodeType, ToNode},
    update::UpdateError,
};

use super::utils::affine::Affine;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, Copy)]
pub struct Coord {
    pub temp: u8,
    pub percent: u8,
}

impl Hash for Coord {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.temp.hash(state);
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
    // unique
    pub name: String,
    // sorted
    // temp unique
    // 0 <= percent <= 100
    #[serde(rename = "coord")]
    pub coords: Vec<Coord>,
    pub input: Option<String>, // Temp or CustomTemp
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            name: Default::default(),
            coords: vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 70,
                    percent: 100,
                },
            ],
            input: Default::default(),
        }
    }
}

impl ToNode for Graph {
    fn to_node(mut self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        let mut deduplicator = HashSet::new();

        for c in &self.coords {
            deduplicator.insert(*c);
        }

        self.coords.retain_mut(|c| {
            if c.percent > 100 {
                warn!("coord percent is superior at 100");
                c.percent = 100;
            }
            deduplicator.contains(c)
        });

        self.coords.sort();

        Node::new(NodeType::Graph(self), app_graph)
    }
}

impl IsValid for Graph {
    fn is_valid(&self) -> bool {
        self.input.is_some() && !self.coords.is_empty()
    }
}

impl Graph {
    pub fn try_new_coord(
        &self,
        temp: &str,
        percent: &str,
    ) -> Result<Coord, Box<dyn std::error::Error>> {
        let temp = temp.parse::<u8>()?;

        let percent = percent.parse::<u8>()?;

        if percent > 100 {
            return Err("Percent > 100".into());
        }

        let coord = Coord { temp, percent };

        if self.coords.binary_search(&coord).is_err() {
            return Err(format!(
                "Can't add create this new coord {}, this temp is already present",
                temp
            )
            .into());
        }

        Ok(coord)
    }

    pub fn get_value(&self, value: Value) -> Result<Value, UpdateError> {
        let dummy_coord = Coord {
            temp: value as u8,
            percent: 0,
        };

        let res = match self.coords.binary_search(&dummy_coord) {
            Ok(index) => self.coords[index].percent as Value,
            Err(index) => {
                if index == 0 {
                    self.coords[index].percent as Value
                } else if index == self.coords.len() {
                    self.coords[index - 1].percent as Value
                } else {
                    let coord1 = &self.coords[index - 1];
                    let coord2 = &self.coords[index];

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
    use crate::config::graph::Coord;

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

        let coords = [coord1, coord2, coord3, coord4];

        let dummy_coord = Coord {
            temp: 50,
            percent: 0,
        };

        let res = coords.binary_search(&dummy_coord);

        match res {
            Ok(index) => {
                println!("use {}", index);
            }
            Err(index) => {
                if index == 0 {
                    println!("use {}", index);
                } else if index == coords.len() {
                    println!("use {}", index - 1);
                } else {
                    println!("use {} and {}", index - 1, index);
                }
            }
        }
        dbg!(&res);
    }
}

// #[derive(PartialEq)]
// enum DupState {
//     Init,
//     Prev { temp: u8 },
//     DuplicateFound,
// }

// self.coords
//     .iter()
//     .fold(DupState::Init, |prev, coord| match prev {
//         DupState::Init => DupState::Prev { temp: coord.temp },
//         DupState::Prev { temp } => {
//             if temp == coord.temp {
//                 DupState::DuplicateFound
//             } else {
//                 DupState::Prev { temp: coord.temp }
//             }
//         }
//         DupState::DuplicateFound => DupState::DuplicateFound,
//     })
//     != DupState::DuplicateFound;
