use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub id: Option<i64>,
    pub title: String,                     // 题目
    pub options: Vec<String>,              // 选项
    pub key: Vec<char>,                    // 答案
    pub wrong_times: Option<i32>,          // 答错的次数
    pub remain_practice_time: Option<i32>, // 剩余联系次数
    pub section_id: Option<i64>,           // 所属Section id
}

/// 检查答案的结果
#[derive(Debug, PartialEq)]
pub enum CheckResult {
    AllCorrect,     // 全对,
    AllWrong,       // 全错
    PartialCorrect, // 部分正确
}

impl Question {
    fn check(&self, answer: &Vec<char>) -> CheckResult {
        let answer_upper: Vec<char> = answer.iter().map(|x| x.to_ascii_uppercase()).collect();
        let key_upper: Vec<char> = self.key.iter().map(|x| x.to_ascii_uppercase()).collect();

        use CheckResult::*;
        if answer_upper.len() > key_upper.len() {
            return AllWrong;
        }

        if answer_upper.len() < key_upper.len() {
            if answer_upper.iter().any(|x| !key_upper.contains(x)) {
                return AllWrong;
            } else {
                return PartialCorrect;
            }
        }

        if answer_upper.iter().any(|x| !key_upper.contains(x)) {
            return AllWrong;
        }

        AllCorrect
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 创建公共的 Question 实例
    fn create_test_question() -> Question {
        Question {
            title: "Test title".to_string(),
            options: vec![
                "AAAAAAAAA".to_string(),
                "BBBBBBBBB".to_string(),
                "CCCCCCCCC".to_string(),
                "DDDDDDDDD".to_string(),
            ],
            key: vec!['A', 'B'],
            wrong_times: None,
            remain_practice_time: None,
            section_id: None,
            id: None,
        }
    }

    #[test]
    fn check_answer_basic() {
        use CheckResult::*;
        let question = create_test_question();

        // 基本判定逻辑
        assert_eq!(question.check(&vec!['A']), PartialCorrect);
        assert_eq!(question.check(&vec!['B']), PartialCorrect);
        assert_eq!(question.check(&vec!['C']), AllWrong);
        assert_eq!(question.check(&vec!['A', 'B']), AllCorrect);
    }

    #[test]
    fn check_answer_case_insensitive() {
        use CheckResult::*;
        let question = create_test_question();

        // 检查小写
        assert_eq!(question.check(&vec!['a']), PartialCorrect);
        assert_eq!(question.check(&vec!['b']), PartialCorrect);
        assert_eq!(question.check(&vec!['c']), AllWrong);
        assert_eq!(question.check(&vec!['a', 'b']), AllCorrect);
    }
}
