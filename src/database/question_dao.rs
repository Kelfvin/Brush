use std::collections::HashMap;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::models::question::Question;

pub struct QuestionDao;

impl QuestionDao {
    pub async fn insert(pool: &SqlitePool, question: &mut Question) -> Result<i64> {
        // 1. 获取 key 的序列化
        let key_str = serde_json::to_string(&question.key)?;
        let option_str = serde_json::to_string(&question.options)?;

        let res =
            sqlx::query("INSERT INTO questions(section_id,title,options,key) VALUES(?, ?, ?, ?)")
                .bind(&question.section_id.unwrap())
                .bind(&question.title)
                .bind(&option_str)
                .bind(&key_str)
                .execute(pool)
                .await?;

        let id = res.last_insert_rowid();
        question.id = Some(id);

        Ok(id)
    }

    pub async fn select_with_question_id(pool: &SqlitePool, id: i64) -> Result<Question> {
        let question = sqlx::query_as::<_, Question>(
            "SELECT *
                FROM questions
                WHERE id=?",
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(question)
    }

    /// 查询所有包含给定所有section_id的question
    /// 返回类型以Map<i64, Vec<Question>>
    pub async fn select_with_section_ids(
        pool: &SqlitePool,
        ids: &Vec<i64>,
    ) -> Result<HashMap<i64, Vec<Question>>> {
        let params: String = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let questions: Vec<Question> = sqlx::query_as(
            "SELECT *
                FROM questions
                WHERE section_id IN (?)",
        )
        .bind(params)
        .fetch_all(pool)
        .await?;

        let mut map: HashMap<i64, Vec<Question>> = HashMap::new();
        for question in questions {
            map.entry(question.section_id.unwrap())
                .or_insert(vec![])
                .push(question);
        }

        Ok(map)
    }

    pub async fn select_with_section_id(pool: &SqlitePool, id: i64) -> Result<Vec<Question>> {
        let questions: Vec<Question> = sqlx::query_as(
            "SELECT *
                FROM questions
                WHERE section_id=?",
        )
        .bind(id)
        .fetch_all(pool)
        .await?;

        Ok(questions)
    }
}
