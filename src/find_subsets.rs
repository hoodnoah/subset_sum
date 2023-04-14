use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LabeledValue {
    pub label: String,
    pub value: f32,
}

impl PartialEq for LabeledValue {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.value == other.value
    }
}

pub fn find_first_subset<'a>(
    input: &'a [LabeledValue],
    target_sum: &'a f32,
) -> Option<Vec<&'a LabeledValue>> {
    let powerset = input.iter().powerset();

    for subset in powerset {
        let sum: f32 = subset.iter().map(|x| (**x).value).sum();
        if sum == *target_sum {
            return Some(subset);
        }
    }
    None
}

#[cfg(test)]
mod find_first_subset_tests {
    use crate::find_subsets::{find_first_subset, LabeledValue};

    #[test]
    fn finds_subset_when_all_are_members() {
        let input = vec![
            LabeledValue {
                label: String::from("a"),
                value: 1.0,
            },
            LabeledValue {
                label: String::from("b"),
                value: 2.0,
            },
            LabeledValue {
                label: String::from("c"),
                value: 3.0,
            },
        ];

        let target_sum = 6.0;

        let result = find_first_subset(&input, &target_sum).unwrap();

        assert_eq!(
            vec![
                &LabeledValue {
                    label: String::from("a"),
                    value: 1.0,
                },
                &LabeledValue {
                    label: String::from("b"),
                    value: 2.0,
                },
                &LabeledValue {
                    label: String::from("c"),
                    value: 3.0,
                },
            ],
            result
        );
    }

    #[test]
    fn test_finds_nothing_when_no_subset() {
        let input = vec![LabeledValue {
            label: String::from("a"),
            value: 1.0,
        }];

        let target = 100.0;

        let result = find_first_subset(&input, &target);

        assert!(result.is_none());
    }

    #[test]
    fn test_finds_subset_when_not_all_members_included() {
        let input = vec![
            LabeledValue {
                label: String::from("a"),
                value: 1.0,
            },
            LabeledValue {
                label: String::from("b"),
                value: 2.0,
            },
            LabeledValue {
                label: String::from("c"),
                value: 3.0,
            },
        ];

        let target = 1.0;

        let result = find_first_subset(&input, &target).unwrap();

        assert_eq!(
            vec![&LabeledValue {
                label: String::from("a"),
                value: 1.0,
            },],
            result
        );
    }
}
