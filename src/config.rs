#[derive(Debug, PartialEq)]
pub struct Config {
    pub input_file_path: String,
    pub target: f32,
}

#[derive(Debug, PartialEq)]
pub enum ConfigError {
    MissingInputFilePath,
    InvalidInputFilePath,
    MissingTarget,
    InvalidTarget,
}

fn is_yaml_file(file_name: &str) -> bool {
    file_name.ends_with(".yaml") || file_name.ends_with(".yml")
}

impl Config {
    pub fn build(args: &mut impl Iterator<Item = String>) -> Result<Config, ConfigError> {
        // skip first arg
        args.next();

        let input_file_path = match args.next() {
            Some(arg) => {
                if is_yaml_file(&arg) {
                    arg
                } else {
                    return Err(ConfigError::InvalidInputFilePath);
                }
            }
            None => return Err(ConfigError::MissingInputFilePath),
        };

        let target = match args.next() {
            Some(arg) => match arg.parse::<f32>() {
                Ok(target) => target,
                Err(_) => return Err(ConfigError::InvalidTarget),
            },
            None => return Err(ConfigError::MissingTarget),
        };

        Ok(Config {
            input_file_path,
            target,
        })
    }
}

#[cfg(test)]
mod config_tests {
    use crate::config;

    #[test]
    fn test_build_with_correct_args() {
        let mut args = vec![
            String::from("program_name"),
            String::from("input_file_path.yaml"),
            String::from("100.34"),
        ]
        .into_iter();

        let expected_config = config::Config {
            input_file_path: String::from("input_file_path.yaml"),
            target: 100.34,
        };

        let config = config::Config::build(&mut args).unwrap();

        assert_eq!(expected_config, config);
    }

    #[test]
    fn test_build_with_no_args() {
        let mut args = vec![String::from("program_name")].into_iter();

        let result = config::Config::build(&mut args);

        assert_eq!(Err(config::ConfigError::MissingInputFilePath), result);
    }

    #[test]
    fn test_build_with_one_arg() {
        let mut args = vec![
            String::from("program_name"),
            String::from("input_file_path.yaml"),
        ]
        .into_iter();

        let result = config::Config::build(&mut args);

        assert_eq!(Err(config::ConfigError::MissingTarget), result);
    }

    #[test]
    fn test_build_with_invalid_file_name() {
        let mut args = vec![
            String::from("program_name"),
            String::from("input_file_path.txt"),
            String::from("100.34"),
        ]
        .into_iter();

        let result = config::Config::build(&mut args);

        assert_eq!(Err(config::ConfigError::InvalidInputFilePath), result);
    }

    #[test]
    fn test_build_with_invalid_target() {
        let mut args = vec![
            String::from("program_name"),
            String::from("input_file_path.yaml"),
            String::from("invalid_target"),
        ]
        .into_iter();

        let result = config::Config::build(&mut args).unwrap_err();

        assert_eq!(config::ConfigError::InvalidTarget, result);
    }
}
