pub mod book_dao;
pub mod question_dao;
pub mod section_dao;

use anyhow::{Ok, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// 获取数据库连接池
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// 创建或打开数据库
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(db_url).await?;
        let db = Database { pool };
        db.create_tables().await?;
        Ok(db)
    }

    /// 在内存中创建，用于测试
    pub async fn open_in_memory() -> Result<Self> {
        let conn = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
        let db = Database { pool: conn };
        db.create_tables().await?;
        Ok(db)
    }

    /// 创建表
    pub async fn create_tables(&self) -> Result<()> {
        // 创建练习册表
        sqlx::query(
            "CREATE TABLE if NOT EXISTS books(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS sections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id INTEGER NOT NULL,
                name TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (book_id) REFERENCES books (id)
            )",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS questions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                section_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                options TEXT NOT NULL,
                key TEXT NOT NULL,
                wrong_times INTEGER DEFAULT 0,
                remain_practice_time INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (section_id) REFERENCES sections (id)
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 获取所有表的信息
    pub async fn get_all_tables_info(&self) -> Result<Vec<String>> {
        let names: Vec<String> =
            sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")
                .fetch_all(&self.pool)
                .await?;

        return Ok(names);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_connection_in_memory() {
        let db = Database::open_in_memory().await.unwrap();
        // 确认表存在
        let mut table_names = db.get_all_tables_info().await.unwrap().clone();
        table_names.sort();

        let mut expected = vec!["books", "questions", "sections", "sqlite_sequence"];
        expected.sort();

        println!("====== Tables ======");
        for name in table_names.iter() {
            println!("{name}");
        }

        assert_eq!(table_names, expected);
    }
}
