#![allow(unused)]

use clap::Parser;

mod util;

const ALGORITHMS: [&str; 6] = ["md5", "sha128", "sha224", "sha256", "sha384", "sha512"];

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long,
        default_value = "md5",
        possible_values = ALGORITHMS,
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
        "sha128" => util::compute_sha128(content),
        "sha224" => util::compute_sha224(content),
        "sha256" => util::compute_sha256(content),
        "sha384" => util::compute_sha384(content),
        "sha512" => util::compute_sha512(content),
        
        // It's impossible to get to this arm. But if you do, please make an issue
        _ => panic!("unknown algorithm {:?}", args.algorithm)
    };

    println!("The {} hash of {:?} is {}", args.algorithm, args.path, result)
}
