pub struct LabeledValue {
    pub label: String,
    pub value: f32,
}

/// Returns true if the sum of the values in the vector is equal to the target.
pub fn is_target_sum(values: &Vec<LabeledValue>, target: &f32) -> bool {
    values.iter().map(|x| x.value).sum::<f32>() == *target
}

#[cfg(test)]
mod is_target_sum_tests {
    use super::LabeledValue;

    #[test]
    fn returns_true_for_correct_sum() {
        let values = vec![
            LabeledValue {
                label: "a".to_string(),
                value: 1.0,
            },
            LabeledValue {
                label: "b".to_string(),
                value: 2.0,
            },
            LabeledValue {
                label: "c".to_string(),
                value: 3.0,
            },
        ];

        let target = 6.0;

        assert_eq!(super::is_target_sum(&values, &target), true);
    }

    #[test]
    fn returns_false_for_incorrect_sum() {
        let values = vec![
            LabeledValue {
                label: "a".to_string(),
                value: 1.0,
            },
            LabeledValue {
                label: "b".to_string(),
                value: 2.0,
            },
            LabeledValue {
                label: "c".to_string(),
                value: 3.0,
            },
        ];

        let target = 7.0;

        assert_eq!(super::is_target_sum(&values, &target), false);
    }
}
