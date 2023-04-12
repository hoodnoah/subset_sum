use itertools::Itertools;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LabeledValue {
    label: String,
    value: f32,
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

fn main() {
    let input = &[
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
    let result = find_first_subset(input, &3.0);

    match result {
        Some(r) => {
            for result in r {
                println!("{:?}", result);
            }
        }
        None => println!("No subset found"),
    }
}
