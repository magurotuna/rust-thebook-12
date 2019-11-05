use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);
    Ok(())
}

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].to_string();
        let filename = args[2].to_string();
        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_new() {
        let args = [
            "hoge".to_string(),
            "piyo".to_string(),
            "fuga".to_string(),
            "foo".to_string(),
            "bar".to_string(),
        ];
        assert_eq!(Err("not enough arguments"), Config::new(&args[0..2]));
        assert_eq!(
            Ok(Config {
                query: "piyo".to_string(),
                filename: "fuga".to_string(),
            }),
            Config::new(&args[0..3])
        );
    }
}
