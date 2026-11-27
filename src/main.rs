use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::new(&args);

    //dbg!("running {}", config);
    println!("searching for {}", config.query);
    println!("in file path {}\n\n", config.file_path);

    let contents = fs::read_to_string(&config.file_path).expect("unable to read file");

    println!("with text:\n {contents} \n\n");
    println!("test: {}", config.test());
}

#[derive(Debug)]
struct Config {
    bin_name: String,
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("all required arguments must be provided")
        }
        let bin_name = &args[0];
        let query = &args[1];
        let file_path = &args[2];

        Config {
            bin_name: bin_name.to_string(),
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }

    fn test(&self) -> String {
        let bin = self.bin_name.clone();
        bin
    }
}
