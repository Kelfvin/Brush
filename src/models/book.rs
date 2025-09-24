use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::section::Section;

#[derive(Debug, Deserialize, Serialize, FromRow, PartialEq)]
pub struct Book {
    pub id: Option<i64>,
    pub name: String,
    #[sqlx(skip)]
    pub sections: Vec<Section>,
}

#[cfg(test)]
mod test {}
