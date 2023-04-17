use crate::find_subsets::LabeledValue;
use prettytable::{row, Table};

/// Given a list of LabeledValue, return a prettytable::Table
pub fn get_table(input: &[&LabeledValue]) -> Table {
    let mut table = Table::new();
    table.add_row(row!["LABEL", "VALUE"]);
    for &item in input {
        table.add_row(row![item.label, item.value]);
    }
    table
}

#[cfg(test)]
mod pretty_print_tests {
    use super::*;
    use crate::find_subsets::LabeledValue;
    use proptest::prelude::*;

    proptest! {
      #[test]
      fn returns_table_with_a_row_per_value(input in prop::collection::vec(any::<f32>(), 1..100)) {
        let labeled_values = input.iter().map(|x| LabeledValue { label: x.to_string(), value: *x }).collect::<Vec<LabeledValue>>();
        let table = get_table(&labeled_values.iter().collect::<Vec<&LabeledValue>>());

        assert_eq!(input.len() + 1, table.len());
      }
    }

    proptest! {
      #[test]
      fn is_idempotent(input in prop::collection::vec(any::<f32>(), 1..100)) {
        let labeled_values = input.iter().map(|x| LabeledValue { label: x.to_string(), value: *x }).collect::<Vec<LabeledValue>>();
        let input = labeled_values.iter().collect::<Vec<&LabeledValue>>();
        let table1 = get_table(&input);
        let table2 = get_table(&input);

        assert_eq!(table1, table2);
      }
    }
}
