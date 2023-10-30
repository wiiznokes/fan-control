use serde::{Deserialize, Serialize};

use super::IsValid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temp {
    pub name: String,
    #[serde(rename = "id")]
    pub hardware_id: Option<String>,

    #[serde(skip)]
    pub hardware_internal_index: Option<usize>,
}

impl IsValid for Temp {
    fn is_valid(&self) -> bool {
        self.hardware_id.is_some() && self.hardware_internal_index.is_some()
    }
}
