use anyhow::Result;
use sqlx::SqlitePool;

use crate::{
    database::section_dao::SectionDao,
    models::{book::Book, section::Section},
};

pub struct BookDao;

impl BookDao {
    /// 插入新的练习册
    pub async fn insert(pool: &SqlitePool, book: &mut Book) -> Result<i64> {
        // 插入练习册
        let res = sqlx::query("INSERT INTO books (name) VALUES (?)")
            .bind(&book.name)
            .execute(pool)
            .await?;

        let book_id = res.last_insert_rowid();
        book.id = Some(book_id);

        for section in &mut book.sections {
            section.book_id = Some(book_id);
        }

        // 插入所有章节
        for section in &mut book.sections {
            SectionDao::insert(pool, section).await?;
        }

        Ok(book_id)
    }

    pub async fn select_with_id(pool: &SqlitePool, id: i64) -> Result<Book> {
        let mut book: Book = sqlx::query_as(
            "SELECT *
                FROM books
                WHERE id=?",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        // 查询该练习册的所有章节
        let sections: Vec<Section> = SectionDao::select_with_book_id(pool, id).await?;

        book.sections = sections;
        Ok(book)
    }
}

#[cfg(test)]
mod test {
    use crate::{database::Database, test_utils::EXAMPLE_JSON};

    use super::*;

    #[tokio::test]
    async fn add_book_and_retrieve() {
        // Build a book object
        let mut book: Book = serde_json::from_str(EXAMPLE_JSON).unwrap();

        // Build a database instance
        let db = Database::open_in_memory().await.unwrap();

        // Insert the book into the database
        let id = BookDao::insert(db.pool(), &mut book).await.unwrap();

        // Retrieve the book from the database
        let retrieved_book = BookDao::select_with_id(db.pool(), id).await.unwrap();

        // assert
        assert_eq!(book, retrieved_book);
    }
}
