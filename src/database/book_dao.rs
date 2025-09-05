use anyhow::Result;
use rusqlite::{Connection, params};

use crate::{database::section_dao::SectionDao, models::book::Book};

pub struct BookDao;

impl BookDao {
    /// 插入新的练习册
    ///
    /// 如果插入过程中发生错误什么都不发生
    pub fn insert(conn: &Connection, book: &mut Book) -> Result<i64> {
        // 开始事物
        let tx = conn.unchecked_transaction()?;

        // 执行插入
        match Self::insert_in_transaction(&tx, book) {
            Ok(book_id) => {
                tx.commit()?;
                Ok(book_id)
            }
            Err(e) => {
                tx.rollback()?;
                Err(e)
            }
        }
    }

    fn insert_in_transaction(conn: &Connection, book: &mut Book) -> Result<i64> {
        // 插入练习册
        conn.execute(
            "INSERT INTO practice_books (name) VALUES (?1)",
            params![book.name],
        )?;

        let book_id = conn.last_insert_rowid();
        book.id = Some(book_id);

        // 插入所有章节
        for section in &mut book.sections {
            SectionDao::insert(conn, section, book_id)?;
        }

        Ok(book_id)
    }
}
