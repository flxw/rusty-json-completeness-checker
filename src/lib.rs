use serde::de::{DeserializeOwned, Error};
use serde_json::Value;

// TODO
// introspect target class
// traverse given raw json object
// panic if something in target class is not touched

// after that:
// try to find place in deserialize trait that makes the traversal across the json payload
// patch to panic/error if something was not considered
pub trait EmptyConstructor {
    fn new_empty() -> Self;

    fn attributes() -> Vec<&'static str>;
}

pub fn parse_json_and_verify_target_completeness<T: EmptyConstructor + DeserializeOwned>(
    json_string: &str,
) -> Result<T, serde_json::Error> {
    let parsed_json: Value = serde_json::from_str(json_string).unwrap();

    let keys_missing_in_model = parsed_json
        .as_object()
        .unwrap()
        .iter()
        .map(|(key, _)| key)
        .filter(|key| !T::attributes().contains(&key.as_str()))
        .collect::<Vec<&String>>();

    if keys_missing_in_model.is_empty() {
        serde_json::from_str(json_string)
    } else {
        let message = format!(
            "A number of keys were transmitted but not covered by the implementation: {:?}",
            keys_missing_in_model
        );
        Err(serde_json::Error::custom(message))
    }
}

#[cfg(test)]
mod tests {
    use crate::parse_json_and_verify_target_completeness;
    use crate::EmptyConstructor;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct Person {
        name: String,
        age: u8,
        phones: Vec<String>,
    }

    impl EmptyConstructor for Person {
        fn new_empty() -> Self {
            Person {
                name: String::new(),
                age: 0,
                phones: vec![],
            }
        }

        fn attributes() -> Vec<&'static str> {
            vec!["name", "age", "phones"]
        }
    }

    #[test]
    fn it_parses_a_complete_model() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;

        // Parse the string of data into serde_json::Value.
        let parsed_person = parse_json_and_verify_target_completeness::<Person>(data);
        assert!(parsed_person.is_ok())
    }

    #[test]
    fn it_prevents_discarding_data_when_parsing_more_data() {
        let data = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ],
            "status": "wanted"
        }"#;

        // Parse the string of data into serde_json::Value.
        let parsed_person = parse_json_and_verify_target_completeness::<Person>(data);
        assert!(parsed_person.is_err());
    }
}
