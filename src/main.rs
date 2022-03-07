#![allow(unused)]

use clap::Parser;

mod util;

const HASH_TYPES: [&'static str; 2] = ["md5", "sha256"];

// CLI utility, which outputs the hash of the given file
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // Type of hash user wishes to use
    #[clap(help = "Type of hash algorithm to use")]
    hash_type: String,
    #[clap(parse(from_os_str), help = "Path to the file")]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read(&args.path).unwrap();
    let result = match args.hash_type.to_ascii_lowercase().as_str() { 
        "md5" => util::compute_md5(content),
        "sha256" => util::compute_sha256(content),
        _ => panic!("Unknown hash type {}", args.hash_type)
    };

    println!("The {} hash of {:?} is {}", args.hash_type, args.path, result)
}
