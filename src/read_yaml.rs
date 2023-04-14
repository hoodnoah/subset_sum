use crate::find_subsets::LabeledValue;

#[derive(Debug)]
pub enum InputReadError {
    YamlParseError(serde_yaml::Error),
}

pub fn read_input_string(input_str: &str) -> Result<Vec<LabeledValue>, InputReadError> {
    let input: Vec<LabeledValue> =
        serde_yaml::from_str(input_str).map_err(|e| InputReadError::YamlParseError(e))?;
    Ok(input)
}

#[cfg(test)]
mod read_yaml_tests {
    use crate::find_subsets::LabeledValue;
    use crate::read_yaml::*;
    #[test]
    fn reads_correctly_formed_input() {
        let input_str = "
      - label: Label 1
        value: 100.0
      - label: Label 2
        value: 200.0
    "
        .to_string();

        let expected_result = vec![
            LabeledValue {
                label: String::from("Label 1"),
                value: 100.0,
            },
            LabeledValue {
                label: String::from("Label 2"),
                value: 200.0,
            },
        ];

        let result = read_input_string(&input_str).unwrap();

        assert_eq!(expected_result, result);
    }

    #[test]
    fn returns_error_for_invalid_yaml() {
        let input_str = "
      - labbl: Label 1
        value: 100.0
      - labbl: Label 2
        value: 200.0";

        let result = read_input_string(&input_str);

        assert!(result.is_err());
    }

    #[test]
    fn returns_error_for_wrong_datatype() {
        let input_str = "
      - label: Label 1
        value: 100.0
      - label: Label 2
        value: 200.0
      - label: Label 3
        value: 'not a number'";

        let result = read_input_string(&input_str);

        assert!(result.is_err());
    }

    #[test]
    fn casts_int_to_float() {
        let input_str = "
      - label: Label 1
        value: 100
      - label: Label 2
        value: 200";

        let expected_result = vec![
            LabeledValue {
                label: String::from("Label 1"),
                value: 100.0,
            },
            LabeledValue {
                label: String::from("Label 2"),
                value: 200.0,
            },
        ];

        let result = read_input_string(&input_str).unwrap();

        assert_eq!(expected_result, result);
    }
}
