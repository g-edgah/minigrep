use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    //dbg!(args);

    let bin_name = &args[0];
    let query = &args[1];
    let file_path = &args[2];

    println!("running {bin_name}");
    println!("searching for {query}");
    println!("in file path {file_path}");
}
