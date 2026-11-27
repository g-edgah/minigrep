use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::new(&args);

    println!("running {0}", config.bin_name);
    println!("searching for {0}", config.query);
    println!("in file path {0}\n\n", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("unable to read file");

    println!("with text:\n {contents}")
}

struct Config {
    bin_name: String,
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let bin_name = &args[0];
        let query = &args[1];
        let file_path = &args[2];

        Config {
            bin_name: bin_name.to_string(),
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
}
