use anyhow::Result;
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// 创建或打开数据库
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        let db = Database { conn };
        db.create_tables().unwrap();
        Ok(db)
    }

    /// 提供链接给 DAO 等使用
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    /// 在内存中创建，用于测试
    pub fn open_in_memory() -> Self {
        let conn = Connection::open_in_memory().unwrap();
        let db = Database { conn };
        db.create_tables().unwrap();
        db
    }

    /// 创建表
    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE if NOT EXISTS practice_books(
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // 章节表
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                book_id INTEGER NOT NULL,
                name TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (book_id) REFERENCES practice_books (id)
            )",
            [],
        )?;

        // 题目表
        self.conn.execute(
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
            [],
        )?;

        Ok(())
    }

    /// 获取所有表的信息
    pub fn get_all_tables_info(&self) -> Result<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name;")?;

        let table_iter = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut tables = Vec::new();
        for table in table_iter {
            tables.push(table?);
        }
        let result = tables.join(", ");

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_connection_in_memory() {
        let db = Database::open_in_memory();

        // 确认表存在
        db.get_all_tables_info().unwrap();
    }
}
