use serde_json::Value;

pub trait JsonHelper {
    fn get_nested_attributes(&self, parsable_string: String) -> &Value;
}

impl JsonHelper for Value {
    fn get_nested_attributes(&self, parsable_string: String) -> &Value {
        let mut root = self;

        for key in parsable_string.split(".") {
            root = root.get(key).unwrap();
        }

        return root;
    }
}

#[cfg(test)]
mod test {
    use crate::utils::json_helper::JsonHelper;
    use serde_json::json;

    #[test]
    fn test_get_nested_attrs() {
        let j = json!({
           "key1": {
                "key2": "value"
            },
            "key3": "value2"
        });

        assert_eq!(
            "value",
            j.get_nested_attributes("key1.key2".to_string())
                .as_str()
                .unwrap()
        );
        assert_eq!(
            "value2",
            j.get_nested_attributes("key3".to_string())
                .as_str()
                .unwrap()
        );
    }
}
