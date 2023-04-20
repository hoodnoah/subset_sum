use crate::find_subsets::LabeledValue;
///! Functionality for reading from and writing to CSV files
/// for convenience.
use csv::Reader;

#[derive(Debug, PartialEq)]
pub enum InputReadError {
    InvalidCSVError,
}

/// Given the string contents of a CSV file, reads
/// them into a vector of LabeledValue structs.
pub fn read_csv_string(input_str: &str) -> Result<Vec<LabeledValue>, InputReadError> {
    let mut csv_reader = Reader::from_reader(input_str.as_bytes());
    csv_reader
        .records()
        .map(|record| {
            let record = record.map_err(|_| InputReadError::InvalidCSVError)?;
            let label = record.get(0).ok_or(InputReadError::InvalidCSVError)?;
            let value = record.get(1).ok_or(InputReadError::InvalidCSVError)?;
            let value = value
                .parse::<f32>()
                .map_err(|_| InputReadError::InvalidCSVError)?;
            Ok(LabeledValue {
                label: label.to_string(),
                value,
            })
        })
        .collect()
}

#[cfg(test)]
mod csv_input_tests {
    use super::*;
    use crate::find_subsets::LabeledValue;
    use proptest::prelude::*;
    use proptest::string::StringParam;

    fn arb_labeled_value() -> impl Strategy<Value = LabeledValue> {
        (
            any_with::<String>(StringParam::from("[a-zA-Z]+")),
            any::<f32>(),
        )
            .prop_map(|(label, value)| LabeledValue { label, value })
    }

    #[test]
    fn test_read_csv() {
        let test_input = "label,value\nLabel 1,100.0\nLabel 2,200.0";
        let expected_output = vec![
            LabeledValue {
                label: String::from("Label 1"),
                value: 100.0,
            },
            LabeledValue {
                label: String::from("Label 2"),
                value: 200.0,
            },
        ];

        let result = read_csv_string(&test_input).unwrap();

        assert_eq!(expected_output, result);
    }

    proptest! {
      #[test]
      fn round_trips_correctly(input in prop::collection::vec(arb_labeled_value(), 0..100)) {
        let input_str = input
          .iter()
          .map(|lv| format!("{},{}", lv.label, lv.value))
          .collect::<Vec<String>>()
          .join("\n");
        let input_str = format!("label,value\n{}", input_str);
        let result = read_csv_string(&input_str).unwrap();
        assert_eq!(input, result);
      }
    }
}
