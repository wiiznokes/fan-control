use std::collections::HashMap;

use hardware::{Hardware, HardwareBridge};

use crate::{
    directories::SettingsManager,
    id::{Id, IdGenerator},
    items::{Control, CustomTemp, Fan, Flat, Graph, Linear, Target, Temp},
    settings::Settings,
};

pub struct AppState {
    pub settings_manager: SettingsManager,
    pub settings: Settings,
    pub hardware_bridge: Box<dyn HardwareBridge>,
    pub hardware: Hardware,
    pub app_graph: AppGraph,
}

pub type Nodes = HashMap<Id, Node>;

pub struct AppGraph {
    pub id_generator: IdGenerator,
    pub nodes: Nodes,
}

impl Default for AppGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl AppGraph {
    pub fn new() -> Self {
        AppGraph {
            id_generator: IdGenerator::new(),
            nodes: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: Id,
    pub node_type: NodeType,
    pub nb_input: NbInput,
    pub input_ids: Vec<Id>,

    pub value: Option<i32>,
}

#[derive(Debug, Clone)]
pub enum NodeType {
    Control(Control),
    Fan(Fan),
    Temp(Temp),
    CustomTemp(CustomTemp),
    Graph(Graph),
    Flat(Flat),
    Linear(Linear),
    Target(Target),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NbInput {
    Fixed(u32),
    Infinity,
}
