use itertools::Itertools;

pub fn non_empty_powerset<'a, T>(sequence: &'a [T]) -> impl Iterator<Item = Vec<&'a T>> {
    let lengths: Vec<usize> = (1..=sequence.len()).collect();

    lengths
        .into_iter()
        .map(move |length| sequence.iter().combinations(length))
        .flatten()
}

#[cfg(test)]
mod non_empty_powerset_tests {
    // testing libraries
    use proptest::prelude::*;

    // module(s) under test
    use super::non_empty_powerset;

    #[test]
    fn returns_expected_combinations() {
        let sequence = vec![1, 2, 3];

        let expected_results = vec![
            vec![&1],
            vec![&2],
            vec![&3],
            vec![&1, &2],
            vec![&1, &3],
            vec![&2, &3],
            vec![&1, &2, &3],
        ];

        assert_eq!(
            non_empty_powerset(&sequence).collect::<Vec<_>>(),
            expected_results,
        );
    }

    proptest! {
        #[test]
        fn returns_expected_num_combinations(sequence in prop::collection::vec(any::<i32>(), 0..=10)) {
            let num_combinations = non_empty_powerset(&sequence).count();
            let expected_num_combinations = 2usize.pow(sequence.len() as u32) - 1;

            assert_eq!(num_combinations, expected_num_combinations);
        }
    }

    proptest! {
        #[test]
        fn all_result_elements_exist_in_input(sequence in prop::collection::vec(any::<i32>(), 0..=10)) {
            let result = non_empty_powerset(&sequence);

            for subset in result {
                for element in subset {
                    assert!(sequence.contains(element));
                }
            }
        }
    }

    proptest! {
        #[test]
        fn all_result_subsets_less_than_or_equal_to_input_length(sequence in prop::collection::vec(any::<i32>(), 0..=10)) {
            let result = non_empty_powerset(&sequence);

            for subset in result {
                assert!(subset.len() <= sequence.len());
            }
        }
    }
}
