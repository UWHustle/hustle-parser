extern crate hustle_parser;

#[cfg(test)]
mod tests {
    use hustle_parser::parse;
    use serde_json::{json, Value};

    #[test]
    fn create_table() {
        let ast_string = parse("CREATE TABLE t (a INT1, b FLOAT4, c IPv4);");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "create_table",
            "name": "t",
            "attributes": [
                {
                    "type": "attribute",
                    "name": "a",
                    "attribute_type": "INT1"
                },
                {
                    "type": "attribute",
                    "name": "b",
                    "attribute_type": "FLOAT4"
                },
                {
                    "type": "attribute",
                    "name": "c",
                    "attribute_type": "IPv4"
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn insert() {
        let ast_string = parse("INSERT INTO t VALUES (1, 2.3, '172.16.254.1');");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "insert",
            "into": {
                "type": "reference",
                "relation": "t"
            },
            "source": {
                "type": "values",
                "values": [
                    {
                        "type": "term",
                        "value": "1"
                    },
                    {
                        "type": "term",
                        "value": "2.3"
                    },
                    {
                        "type": "term",
                        "value": "'172.16.254.1'"
                    }
                ]
            }
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn select() {
        let ast_string = parse("SELECT a FROM t;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "select",
            "from": {
                "type": "reference",
                "relation": "t"
            },
            "select": [
                {
                    "type": "reference",
                    "attribute": "a"
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn cartesian() {
        let ast_string = parse("SELECT t.a, u.b FROM t, u;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "select",
            "from": {
                "type": "join",
                "left": {
                    "type": "reference",
                    "relation": "t"
                },
                "right": {
                    "type": "reference",
                    "relation": "u"
                }
            },
            "select": [
                {
                    "type": "reference",
                    "relation": "t",
                    "attribute": "a"
                },
                {
                    "type": "reference",
                    "relation": "u",
                    "attribute": "b"
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn comma_join() {
        let ast_string = parse("SELECT t.a, u.b FROM t, u WHERE t.a = u.b;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "select",
            "from": {
                "type": "join",
                "left": {
                    "type": "reference",
                    "relation": "t"
                },
                "right": {
                    "type": "reference",
                    "relation": "u"
                }
            },
            "where": {
                "type": "function",
                "name": "eq",
                "arguments": [
                    {
                        "type": "reference",
                        "relation": "t",
                        "attribute": "a"
                    },
                    {
                        "type": "reference",
                        "relation": "u",
                        "attribute": "b"
                    }
                ]
            },
            "select": [
                {
                    "type": "reference",
                    "relation": "t",
                    "attribute": "a"
                },
                {
                    "type": "reference",
                    "relation": "u",
                    "attribute": "b"
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn keyword_join() {
        let ast_string = parse("SELECT t.a FROM t JOIN u ON t.a = u.b;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "select",
            "from": {
                "type": "join",
                "left": {
                    "type": "reference",
                    "relation": "t"
                },
                "right": {
                    "type": "reference",
                    "relation": "u"
                },
                "predicate": {
                    "type": "function",
                    "name": "eq",
                    "arguments": [
                        {
                            "type": "reference",
                            "relation": "t",
                            "attribute": "a"
                        },
                        {
                            "type": "reference",
                            "relation": "u",
                            "attribute": "b"
                        }
                    ]
                }
            },
            "select": [
                {
                    "type": "reference",
                    "relation": "t",
                    "attribute": "a"
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn update() {
        let ast_string = parse("UPDATE t SET a = 2, b = 3;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "update",
            "relation": {
                "type": "reference",
                "relation": "t"
            },
            "assignments": [
                {
                    "type": "assignment",
                    "attribute": "a",
                    "value": {
                        "type": "term",
                        "value": "2"
                    }
                },
                {
                    "type": "assignment",
                    "attribute": "b",
                    "value": {
                        "type": "term",
                        "value": "3"
                    }
                }
            ]
        });
        assert_eq!(ast, expected);
    }

    #[test]
    fn update_where() {
        let ast_string = parse("UPDATE t SET a = 2 WHERE a = 1;");
        let ast: Value = serde_json::from_str(&ast_string).unwrap();
        let expected = json!({
            "type": "update",
            "relation": {
                "type": "reference",
                "relation": "t"
            },
            "where": {
                "type": "function",
                "name": "eq",
                "arguments": [
                    {
                        "type": "reference",
                        "attribute": "a"
                    },
                    {
                        "type": "term",
                        "value": "1"
                    }
                ]
            },
            "assignments": [
                {
                    "type": "assignment",
                    "attribute": "a",
                    "value": {
                        "type": "term",
                        "value": "2"
                    }
                }
            ]
        });
        assert_eq!(ast, expected);
    }
}
