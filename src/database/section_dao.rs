use anyhow::Result;
use rusqlite::{Connection, params};

use crate::{
    database::question_dao::QuestionDao,
    models::{question::Question, section::Section},
};

pub struct SectionDao;
impl SectionDao {
    pub fn insert(conn: &Connection, section: &mut Section, book_id: i64) -> Result<i64> {
        conn.execute(
            "INSERT INTO sections(book_id,name) VALUES(?1,?2)",
            params![book_id, section.name],
        )?;

        let id = conn.last_insert_rowid();
        section.id = Some(id);

        // 插入所有的题目
        for question in &mut section.questions {
            QuestionDao::insert(conn, question, id)?;
        }

        Ok(id)
    }

    pub fn select_with_section_id(conn: &Connection, id: i64) -> Result<Section> {
        let mut stmt = conn.prepare(
            "SELECT *
                FROM sections
                WHERE id=?1",
        )?;

        let (name, book_id) = stmt.query_row([id], |row| {
            Ok((row.get::<_, String>("name")?, row.get::<_, i64>("book_id")?))
        })?;

        // 查询该章节下的所有题目
        let questions: Vec<Question> = QuestionDao::select_with_section_id(conn, id)?;

        let section = Section {
            id: Some(id),
            name: name,
            questions: questions,
            book_id: Some(book_id),
        };

        Ok(section)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        database::{book_dao::BookDao, database::Database},
        models::book::Book,
        test_utils::EXAMPLE_JSON,
    };

    #[test]
    fn add_book_and_retrieve() {
        let mut book: Book = serde_json::from_str(EXAMPLE_JSON).unwrap();

        let db = Database::open_in_memory();

        let id = BookDao::insert(db.conn(), &mut book).unwrap();

        println!("id: {}", id);
    }
}
