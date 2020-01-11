use minigrep::Config;
use std::env;
use std::process;

fn main() {
    //
    // Accepting Command Line Arguments
    //
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // ロジックを `parse_config` に譲渡
    // let query = &args[1];
    // let filename = &args[2];

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // ------------------------

    //
    // Reading a File
    //
    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
