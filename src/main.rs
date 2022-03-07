#![allow(unused)]

use clap::Parser;

mod util;

const POSSIBLE_ALGORITHMS: [&'static str; 2] = ["md5", "sha256"];

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long,
        default_value = "md5",
        possible_values = POSSIBLE_ALGORITHMS,
        help = "Hash algorithm to use"
    )]
    algorithm: String,
    #[clap(parse(from_os_str), help = "Path to the file")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read(&args.path).unwrap();
    let result = match args.algorithm.as_str() { 
        "md5" => util::compute_md5(content),
        "sha256" => util::compute_sha256(content),
        _ => panic!("how? unknown algorithm {}", args.algorithm)
    };

    println!("The {} hash of {:?} is {}", args.algorithm, args.path, result)
}
