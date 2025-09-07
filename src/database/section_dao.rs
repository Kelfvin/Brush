use std::collections::HashMap;

use anyhow::{Ok, Result};
use sqlx::SqlitePool;

use crate::{
    database::question_dao::QuestionDao,
    models::{question::Question, section::Section},
};

pub struct SectionDao;
impl SectionDao {
    pub async fn insert(pool: &SqlitePool, section: &mut Section) -> Result<i64> {
        let res = sqlx::query("INSERT INTO sections(book_id,name) VALUES(?1,?2)")
            .bind(section.book_id.unwrap())
            .bind(&section.name)
            .execute(pool)
            .await?;

        let id = res.last_insert_rowid();
        section.id = Some(id);

        // 修改所有题目的 section_id
        for question in &mut section.questions {
            question.section_id = Some(id);
        }

        // 插入所有的题目
        for question in &mut section.questions {
            QuestionDao::insert(pool, question).await?;
        }

        Ok(id)
    }

    pub async fn select_by_id(pool: &SqlitePool, id: i64) -> Result<Section> {
        let mut section: Section =
            sqlx::query_as::<_, Section>("SELECT * FROM sections WHERE id=?1")
                .bind(id)
                .fetch_one(pool)
                .await?;

        let questions: Vec<Question> =
            sqlx::query_as("SELECT * FROM questions WHERE section_id=?1")
                .bind(id)
                .fetch_all(pool)
                .await?;

        section.questions = questions;

        Ok(section)
    }

    pub async fn select_with_book_id(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        id: i64,
    ) -> Result<Vec<Section>> {
        let mut sections: Vec<Section> = sqlx::query_as("SELECT * FROM sections WHERE book_id=?1")
            .bind(id)
            .fetch_all(pool)
            .await?;

        let section_ids = sections
            .iter()
            .map(|section| section.id.unwrap_or(-1)) // 实际不可能
            .collect::<Vec<i64>>();

        let problems_group: HashMap<i64, Vec<Question>> =
            QuestionDao::select_with_section_ids(pool, &section_ids).await?;

        for (section_id, questions) in problems_group {
            if let Some(section) = sections.iter_mut().find(|s| s.id.unwrap() == section_id) {
                section.questions = questions;
            }
        }

        Ok(sections)
    }
}
