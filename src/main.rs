use std::{fs, io::Read};
use subset_sum::config::Config;
use subset_sum::pretty_print;

fn main() {
    let config = Config::build(&mut std::env::args()).unwrap();

    let mut input_file_contents = String::new();
    fs::File::open(&config.input_file_path)
        .unwrap()
        .read_to_string(&mut input_file_contents)
        .unwrap();

    let input = subset_sum::read_yaml::read_input_string(&input_file_contents).unwrap();

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
