use std::collections::HashMap;

use crate::items::{Control, CustomTemp, Fan, Flat, Graph, Linear, Target, Temp};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HardwareType {
    Control,
    Fan,
    Temp,
}

#[derive(Debug, Clone)]
enum NodeType {
    Control(Control),
    Fan(Fan),
    Temp(Temp),
    CustomTemp(CustomTemp),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear),
    Target(Target),
}

#[derive(Debug, Clone)]
struct Node {
    node_type: NodeType,
    max_input: u32,
    max_ouput: u32,
    input_id: Vec<u64>,
    output_id: Vec<u64>,

    value: Option<i32>,
    info: String,
}

struct AppGraph {
    nodes: HashMap<u64, Node>,
}

#[derive(Debug, Clone)]
enum UpdateError {
    NodeNotFound,
}

impl AppGraph {
    pub fn update(&mut self, _node: u64) -> Result<(), UpdateError> {
        Ok(())
    }
}
