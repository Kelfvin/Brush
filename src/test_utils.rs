pub const EXAMPLE_JSON: &str = r#"
        {
            "name": "Test Book",
            "sections": [
                {
                    "name": "第一章测试题",
                    "questions": [
                        {
                            "title": "测试题目1",
                            "options": ["A", "B", "C", "D"],
                            "key": ["A"],
                            "type": "SS"
                        },
                        {
                            "title": "测试题目2",
                            "options": ["A", "B", "C", "D"],
                            "key": ["A"],
                            "type": "MS"
                        }
                    ]
                },
                {
                    "name": "第二章测试题",
                    "questions": [
                        {
                            "title": "测试题目1",
                            "options": ["A", "B", "C", "D"],
                            "key": ["A"],
                            "type": "SS"
                        },
                        {
                            "title": "测试题目2",
                            "options": ["A", "B", "C", "D"],
                            "key": ["A"],
                            "type": "MS"
                        }
                    ]
                }
            ]
        }
        "#;
