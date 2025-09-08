pub mod book_dao;
pub mod question_dao;
pub mod section_dao;
pub mod utils_dao;

use crate::database::utils_dao::UtilsDao;
use anyhow::{Ok, Result};
use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};

#[derive(Debug)]
pub struct Database {
    pool: SqlitePool,
    // static version i64
}

impl Database {
    /// 获取数据库连接池
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    async fn init(&self) -> Result<()> {
        self.create_tables().await?;
        if UtilsDao::is_first_time_launch(&self.pool).await? {
            UtilsDao::insert_sample_book(&self.pool).await?;
        }

        UtilsDao::login_history(&self.pool).await?;

        Ok(())
    }

    /// 指定路径创建或打开数据库，用于生产环境
    pub async fn new(db_url: &str) -> Result<Self> {
        let pool = SqlitePoolOptions::new().connect(db_url).await?;
        let db = Database { pool };
        db.init().await?;
        Ok(db)
    }

    /// 在内存中创建，用于测试
    pub async fn open_in_memory() -> Result<Self> {
        let conn = SqlitePoolOptions::new().connect("sqlite::memory:").await?;
        let db = Database { pool: conn };
        db.init().await?;
        Ok(db)
    }

    async fn update_version(&self, version: i64) -> Result<()> {
        sqlx::query("INSERT INTO version (version) VALUES (?);")
            .bind(version)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// 创建记录用户打开应用的历史表
    async fn create_history_table(&self) -> Result<()> {
        // create history table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                last_use_time DATETIME DEFAULT CURRENT_TIMESTAMP)",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_version_table(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS version (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                version TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_curren_version(&self) -> Result<i64> {
        let version: Option<i64> = sqlx::query_scalar("SELECT MAX(version) FROM version;")
            .fetch_one(&self.pool)
            .await?;

        Ok(version.unwrap_or(0))
    }

    async fn create_books_table(&self) -> Result<()> {
        // Create books table
        sqlx::query(
            "CREATE TABLE if NOT EXISTS books(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn create_sections_table(&self) -> Result<()> {
        // Create sections table
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

        Ok(())
    }

    async fn create_questions_table(&self) -> Result<()> {
        // Create questions table
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS questions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                section_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                options TEXT NOT NULL,
                key TEXT NOT NULL,
                wrong_times INTEGER DEFAULT 0,
                remain_practice_times INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (section_id) REFERENCES sections (id)
            )",
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// 创建表
    pub async fn create_tables(&self) -> Result<()> {
        // 1. 创建 version table
        self.create_version_table().await?;

        // 2. 查询原数据库版本
        let curren_version = self.get_curren_version().await?;

        // 3. 根据不同的 version 执行不同的升级操作
        if curren_version < 1 {
            // 初始版本
            // 创建 history 表
            self.create_history_table().await?;

            // 创建 books 表
            self.create_books_table().await?;

            // 创建 sections 表
            self.create_sections_table().await?;

            // 创建 questions 表
            self.create_questions_table().await?;

            self.update_version(1).await?;
        }

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
        let db = Database::open_in_memory()
            .await
            .expect("创建内存数据库失败");
        // 确认表存在
        let mut table_names = db.get_all_tables_info().await.unwrap().clone();
        table_names.sort();

        let mut expected = vec!["books", "questions", "sections", "sqlite_sequence"];
        expected.sort();

        println!("====== Tables ======");
        for name in table_names.iter() {
            println!("{name}");
        }
    }
}
