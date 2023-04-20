use std::{fs, io::Read};
use subset_sum::config::Config;
use subset_sum::pretty_print;

enum FileType {
    CSV,
    YAML,
}

fn determine_file_type(file_path: &str) -> FileType {
    let file_path = file_path.to_lowercase();
    if file_path.ends_with(".csv") {
        FileType::CSV
    } else if file_path.ends_with(".yaml") || file_path.ends_with(".yml") {
        FileType::YAML
    } else {
        panic!("File type not recognized.")
    }
}

fn main() {
    let config = Config::build(&mut std::env::args()).unwrap();

    let mut input_file_contents = String::new();
    fs::File::open(&config.input_file_path)
        .unwrap()
        .read_to_string(&mut input_file_contents)
        .unwrap();

    let input = match determine_file_type(&config.input_file_path) {
        FileType::CSV => subset_sum::csv_input::read_csv_string(&input_file_contents).unwrap(),
        FileType::YAML => subset_sum::read_yaml::read_input_string(&input_file_contents).unwrap(),
    };

    let result = subset_sum::find_subsets::find_first_subset(&input, &config.target);

    match result {
        Some(r) => {
            println!("{} is comprised of the following subset:", config.target);
            pretty_print::get_table(&r[..]).printstd();
        }
        None => {
            println!("No valid subset found.")
        }
    }
}
