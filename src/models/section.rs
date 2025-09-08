use crate::models::question::Question;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow, PartialEq)]
pub struct Section {
    pub id: Option<i64>,
    pub name: String,
    #[sqlx(skip)]
    pub questions: Vec<Question>,
    pub book_id: Option<i64>, // 关联到 Book
}

#[cfg(test)]
mod test {

    use crate::{database::utils_dao::EXAMPLE_JSON, models::book::Book};

    /// 测试能否解析成功文件
    #[test]
    fn can_parse() {
        let book: Book = serde_json::from_str(EXAMPLE_JSON).unwrap();
        assert_eq!(book.name, "Hello Brush");
    }
}
