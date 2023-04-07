use itertools::Itertools;

fn all_combinations<'a, T>(sequence: &'a [T]) -> impl Iterator<Item = Vec<&'a T>> {
    let lengths: Vec<usize> = (1..=sequence.len()).collect();

    lengths
        .into_iter()
        .map(move |length| sequence.iter().combinations(length))
        .flatten()
}

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
        all_combinations(&sequence).collect::<Vec<_>>(),
        expected_results,
    );
}

fn main() {
    println!("Hello, world!");
}
