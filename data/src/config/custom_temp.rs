use serde::{Deserialize, Serialize};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CustomTempType {
    Min,
    Max,
    Average,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CustomTemp {
    pub name: String,
    pub kind: CustomTempType,
    pub input: Vec<String>, // Temp
}

impl IsValid for CustomTemp {
    fn is_valid(&self) -> bool {
        !self.input.is_empty()
    }
}
