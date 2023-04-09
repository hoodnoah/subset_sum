#[derive(Debug, Clone)]
pub struct LabeledValue {
    pub label: String,
    pub value: f32,
}

const EQUALITY_THRESHOLD: f32 = 1.0e-6;

/// Determines if two floats are close enough to be considered equal.
fn is_close_enough(a: &f32, b: &f32) -> bool {
    (a - b).abs() <= EQUALITY_THRESHOLD
}

/// Returns true if the sum of the values in the vector is equal to the target.
pub fn is_target_sum(values: &Vec<LabeledValue>, target: &f32) -> bool {
    let sum = values.iter().map(|x| x.value).sum::<f32>();

    is_close_enough(&sum, target)
}

#[cfg(test)]
mod is_target_sum_tests {
    // testing libraries
    use proptest::prelude::*;

    // module(s) under test
    use super::LabeledValue;

    proptest! {
    #[test]

    fn returns_true_for_correct_sum(input in prop::collection::vec(any::<f32>(), 1..=10)) {
        let values = input.iter().map(|x| LabeledValue { label: "".to_string(), value: *x }).collect::<Vec<_>>();
        let target = input.iter().sum::<f32>();

        assert_eq!(super::is_target_sum(&values, &target), true);
    }}

    proptest! {
    #[test]
    fn returns_false_for_incorrect_sum(input in prop::collection::vec(any::<f32>(), 1..=10)) {
        let values = input.iter().map(|x| LabeledValue { label: "".to_string(), value: *x }).collect::<Vec<_>>();
        let target = input.iter().map(|x| x).sum::<f32>() / 2.0 + 1.0;

        assert_eq!(super::is_target_sum(&values, &target), false);
    }
    }
}
