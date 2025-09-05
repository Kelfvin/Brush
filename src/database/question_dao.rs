use anyhow::Result;
use rusqlite::{Connection, params};

use crate::models::question::Question;

pub struct QuestionDao;

impl QuestionDao {
    pub fn insert(conn: &Connection, question: &mut Question, section_id: i64) -> Result<i64> {
        // 1. 获取 key 的序列化
        let key_str = serde_json::to_string(&question.key)?;
        let option_str = serde_json::to_string(&question.options)?;

        conn.execute(
            "INSERT INTO questions(section_id,title,options,key) VALUES(?1, ?2, ?3, ?4)",
            params![section_id, question.title, option_str, key_str],
        )?;

        let id = conn.last_insert_rowid();
        question.id = Some(id);

        Ok(id)
    }

    pub fn select_with_question_id(conn: &Connection, id: i64) -> Result<Question> {
        let mut stmt = conn.prepare(
            "SELECT *
                FROM questions
                WHERE id=?1",
        )?;

        let (title, options_json, key_json, wrong_times, remain_practice_time, section_id) =
            stmt.query_row([id], |row| {
                Ok((
                    row.get::<_, String>("title")?,
                    row.get::<_, String>("options")?, // Note: column name should match your schema
                    row.get::<_, String>("key")?,     // Note: column name should match your schema
                    row.get::<_, Option<i32>>("wrong_times")?,
                    row.get::<_, Option<i32>>("remain_practice_time")?,
                    row.get::<_, Option<i64>>("section_id")?,
                ))
            })?;

        let question = Question {
            id: Some(id),
            title: title,
            options: serde_json::from_str(&options_json)?,
            key: serde_json::from_str(&key_json)?,
            wrong_times: wrong_times,
            remain_practice_time: remain_practice_time,
            section_id: section_id,
        };

        Ok(question)
    }

    pub fn select_with_section_id(conn: &Connection, id: i64) -> Result<Vec<Question>> {
        let mut stmt = conn.prepare(
            "SELECT *
                FROM questions
                WHERE section_id=?1",
        )?;

        let question_param_iter = stmt.query_map([id], |row| {
            Ok((
                row.get::<_, i64>("id")?,
                row.get::<_, String>("title")?,
                row.get::<_, String>("options")?, // Note: column name should match your schema
                row.get::<_, String>("key")?,     // Note: column name should match your schema
                row.get::<_, Option<i32>>("wrong_times")?,
                row.get::<_, Option<i32>>("remain_practice_time")?,
                row.get::<_, i64>("section_id")?,
            ))
        })?;

        let mut questions: Vec<Question> = Vec::new();
        for question_param in question_param_iter {
            let (id, title, options_json, key_json, wrong_times, remain_practice_time, section_id) =
                question_param?;
            let question = Question {
                id: Some(id),
                title: title,
                options: serde_json::from_str(&options_json)?,
                key: serde_json::from_str(&key_json)?,
                wrong_times: wrong_times,
                remain_practice_time: remain_practice_time,
                section_id: Some(section_id),
            };
            questions.push(question);
        }
        Ok(questions)
    }
}
