use std::collections::HashMap;

use anyhow::{Ok, Result};
use itertools::Itertools;
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

        let section_id = res.last_insert_rowid();
        section.id = Some(section_id);

        // 修改所有题目的 section_id
        for question in &mut section.questions {
            question.section_id = Some(section_id);
        }

        // 插入所有的题目
        for question in &mut section.questions {
            QuestionDao::insert(pool, question).await?;
        }

        Ok(section_id)
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

    /// 根据练习册 ID 查询所有章节
    pub async fn select_with_book_id(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        id: i64,
    ) -> Result<Vec<Section>> {
        // 查询给定 id 的 book 的所有 sections
        let mut sections: Vec<Section> = sqlx::query_as("SELECT * FROM sections WHERE book_id=?")
            .bind(id)
            .fetch_all(pool)
            .await?;

        // 收集所有的 section 的 ID 用于查询 questions
        let section_ids = sections
            .iter()
            .map(|section| section.id.unwrap_or(-1)) // 实际不可能
            .collect::<Vec<i64>>();

        // query questions by section_ids
        let problems_group: HashMap<i64, Vec<Question>> =
            QuestionDao::select_with_section_ids(pool, &section_ids).await?;

        for (section_id, questions) in problems_group {
            if let Some(section) = sections.iter_mut().find(|s| s.id.unwrap() == section_id) {
                section.questions = questions;
            }
        }

        Ok(sections)
    }

    pub async fn selection_group_by_book_ids(
        pool: &sqlx::Pool<sqlx::Sqlite>,
        book_ids: &[i64],
    ) -> Result<HashMap<i64, Vec<Section>>> {
        if book_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let mut query_builder =
            sqlx::QueryBuilder::new("SELECT * FROM sections WHERE book_id IN (");

        let mut separated = query_builder.separated(", ");

        for book_id in book_ids {
            separated.push_bind(book_id);
        }

        separated.push_unseparated(")");

        let sections: Vec<Section> = query_builder.build_query_as().fetch_all(pool).await?;

        let map = sections
            .into_iter()
            .chunk_by(
                |sec| sec.book_id.unwrap_or(-1), // 实际不可能
            )
            .into_iter()
            .map(|(book_id, group)| (book_id, group.collect()))
            .collect();

        Ok(map)
    }
}
