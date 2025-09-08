use std::collections::HashMap;

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

    /// 根据id查询section
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

    /// 查询所有的 book
    ///
    /// 惰性查询，其questions字段为空
    pub async fn select_all(pool: &SqlitePool) -> Result<Vec<Book>> {
        let books: Vec<Book> = sqlx::query_as("SELECT * FROM books")
            .fetch_all(pool)
            .await?;
        Ok(books)
    }

    /// 查询所有的book
    ///
    /// > [!WARNING]:
    /// >
    /// > 不建议一次性查询所有的book，建议使用惰性查询
    pub async fn select_all_entity(pool: &SqlitePool) -> Result<Vec<Book>> {
        // 查询所有的 books （from row）
        let mut books: Vec<Book> = sqlx::query_as("SELECT * FROM books")
            .fetch_all(pool)
            .await?;

        let book_ids: Vec<i64> = books.iter().filter_map(|b| b.id).collect();

        // 查询所有的 sections （from row）
        let mut sections_group: HashMap<i64, Vec<Section>> =
            SectionDao::selection_group_by_book_ids(pool, &book_ids).await?;

        // 提取所有的 section_ids
        let section_ids: Vec<i64> = sections_group
            .values()
            .flatten()
            .filter_map(|s| s.id)
            .collect();

        // 查询所有的 questions
        let mut questions_group =
            crate::database::question_dao::QuestionDao::select_with_section_ids(pool, &section_ids)
                .await?;

        // 关联 questions 到 sections
        for sections in sections_group.values_mut() {
            for section in sections {
                if let Some(section_id) = section.id {
                    if let Some(questions) = questions_group.remove(&section_id) {
                        section.questions = questions;
                    }
                }
            }
        }

        // 关联 sections 到 books
        for book in &mut books {
            if let Some(book_id) = book.id {
                if let Some(sections) = sections_group.remove(&book_id) {
                    book.sections = sections;
                }
            }
        }

        Ok(books)
    }
}

#[cfg(test)]
mod test {
    use crate::{database::Database, database::utils_dao::EXAMPLE_JSON};

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

    #[tokio::test]
    async fn fetch_all() {
        let db = Database::open_in_memory().await.unwrap();

        let books = BookDao::select_all_entity(db.pool()).await.unwrap();

        dbg!(&books);
    }
}
