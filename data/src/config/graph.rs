use std::{collections::BTreeSet, hash::Hash, vec};

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

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Graph {
    // unique
    pub name: String,
    // sorted
    // temp unique
    // 0 <= percent <= 100
    #[serde(rename = "coord")]
    pub coords: BTreeSet<Coord>,
    pub input: Option<String>, // Temp or CustomTemp
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.input == other.input
            && self.coords.len() == other.coords.len()
            && self
                .coords
                .iter()
                .zip(&other.coords)
                .all(|(a, b)| a.exact_same(b))
    }
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
            ]
            .into_iter()
            .collect(),
            input: Default::default(),
        }
    }
}

impl ToNode for Graph {
    fn to_node(mut self, app_graph: &mut AppGraph, _hardware: &Hardware) -> Node {
        let mut deduplicator = BTreeSet::new();

        for mut c in self.coords.clone() {
            if c.percent > 100 {
                warn!("coord percent is superior to 100");
                c.percent = 100;
            }
            if !deduplicator.insert(c) {
                warn!("2 coords share the same temp");
            }
        }

        self.coords = deduplicator.into_iter().collect();

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

        if self.coords.contains(&coord) {
            return Err(format!(
                "Can't create this new coord {}, this temp is already present",
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

        let res = match self.coords.get(&dummy_coord) {
            Some(c) => c.percent as Value,
            None => {
                let lower_bound = self.coords.range(..=dummy_coord).next_back();
                let upper_bound = self.coords.range(dummy_coord..).next();

                match (lower_bound, upper_bound) {
                    (Some(coord), None) | (None, Some(coord)) => coord.percent as Value,
                    (Some(coord1), Some(coord2)) => Affine {
                        xa: coord1.temp.into(),
                        ya: coord1.percent.into(),
                        xb: coord2.temp.into(),
                        yb: coord2.percent.into(),
                    }
                    .calcule(value) as Value,

                    _ => panic!("internal error: no value for graph"),
                }
            }
        };

        Ok(res)
    }

    pub fn add_coord(&mut self, new: Coord) {
        self.coords.insert(new);
    }
    pub fn remove_coord(&mut self, coord: &Coord) {
        self.coords.remove(coord);
    }
    pub fn replace_coord(&mut self, prev: &Coord, new: Coord) {
        self.remove_coord(prev);
        self.add_coord(new);
    }
}

#[cfg(test)]
mod test {
    use crate::{config::graph::Coord, node::IsValid};

    use super::Graph;

    #[test]
    fn test_logic() {
        let graph = Graph {
            name: "name".into(),
            coords: vec![
                Coord {
                    temp: 10,
                    percent: 10,
                },
                Coord {
                    temp: 20,
                    percent: 30,
                },
                Coord {
                    temp: 25,
                    percent: 20,
                },
                Coord {
                    temp: 30,
                    percent: 25,
                },
                Coord {
                    temp: 40,
                    percent: 5,
                },
            ]
            .into_iter()
            .collect(),
            input: None,
        };

        graph.is_valid();

        assert_eq!(graph.get_value(9).unwrap(), 10);
        assert_eq!(graph.get_value(50).unwrap(), 5);

        assert_eq!(graph.get_value(22).unwrap(), 26);
        assert_eq!(graph.get_value(27).unwrap(), 22);
        assert_eq!(graph.get_value(35).unwrap(), 15);
    }
}
