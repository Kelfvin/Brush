use anyhow::Result;
use sqlx::SqlitePool;

pub struct UtilsDao;

impl UtilsDao {
    pub async fn login_history(pool: &SqlitePool) -> Result<i64> {
        let res = sqlx::query("INSERT INTO history (last_use_time) VALUES (CURRENT_TIMESTAMP);")
            .execute(pool)
            .await?;

        let id = res.last_insert_rowid();
        Ok(id)
    }

    pub async fn is_first_time_launch(pool: &SqlitePool) -> Result<bool> {
        // 检查 history 表是否存在记录
        let result = sqlx::query("SELECT 1 FROM history LIMIT 1")
            .fetch_one(pool)
            .await;

        match result {
            Ok(_) => Ok(false), // 存在，说明不是第一次启动
            _ => Ok(true),      // 不存在，说明是第一次启动
        }
    }

    /// 插入样例数据
    pub async fn insert_sample_book(pool: &SqlitePool) -> Result<i64> {
        let mut book: crate::models::book::Book = serde_json::from_str(EXAMPLE_JSON).unwrap();
        let book_id = crate::database::book_dao::BookDao::insert(pool, &mut book).await?;
        Ok(book_id)
    }
}

pub const EXAMPLE_JSON: &str = r#"
        {
            "name": "Hello Brush",
            "sections": [
                {
                    "name": "第一章测试题",
                    "questions": [
                        {
                            "title": "我国哪一事件标志着新中国的成立？",
                            "options": [
                                "五四运动", 
                                "抗日战争胜利", 
                                "中华人民共和国成立典礼", 
                                "辛亥革命"
                                ],
                            "key": ["C"],
                            "type": "SS"
                        },
                        {
                            "title": "秦始皇统一六国后，统一了文字、度量衡和货币。",
                            "options": ["对", "错"],
                            "key": ["A"],
                            "type": "TF"
                        }
                    ]
                },
                {
                    "name": "第二章测试题",
                    "questions": [
                        {
                            "title": "1+1=?",
                            "options": ["0", "1", "2", "3"],
                            "key": ["A"],
                            "type": "SS"
                        },
                        {
                            "title": "以下哪些选项是编程语言？",
                            "options": ["C++", "Java", "SpringBoot", "Rust"],
                            "key": ["A", "B", "D"],
                            "type": "MS"
                        }
                    ]
                }
            ]
        }
        "#;

#[cfg(test)]
mod test {
    use crate::database::Database;

    #[tokio::test]
    async fn test_first_time_launch_insert_sample() {
        let db = Database::open_in_memory().await.unwrap();
    }
}
