use serde::{Deserialize, Serialize};

use crate::models::question::Question;

#[derive(Debug, Deserialize, Serialize)]
pub struct Section {
    pub id: Option<i64>,
    pub name: String,
    pub questions: Vec<Question>,

    pub book_id: Option<i64>, // 关联到 Book
}

#[cfg(test)]
mod test {

    use crate::{models::book::Book, test_utils::EXAMPLE_JSON};

    /// 测试能否解析成功文件
    #[test]
    fn can_parse() {
        let book: Book = serde_json::from_str(EXAMPLE_JSON).unwrap();
        assert_eq!(book.name, "Test Book");
    }
}
