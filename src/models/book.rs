use serde::{Deserialize, Serialize};

use crate::models::section::Section;

#[derive(Debug, Deserialize, Serialize)]
pub struct Book {
    pub id: Option<i64>,
    pub name: String,
    pub sections: Vec<Section>,
}

#[cfg(test)]
mod test {}
