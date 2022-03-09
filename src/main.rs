use clap::Parser;
use colored::Colorize;

mod util;

const ALGORITHMS: [&str; 6] = ["md5", "sha128", "sha224", "sha256", "sha384", "sha512"];

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long, default_value = "md5", possible_values = ALGORITHMS, help = "Hash algorithm to use")]
    algorithm: String,
    #[clap(parse(from_os_str), help = "Path to the file")]
    path: std::path::PathBuf,
    #[clap(short, long, help = "Print raw digest")]
    unformatted: bool,
}

fn main() {
    let args = Cli::parse();

    let file = std::fs::read(&args.path);
    let file = match file {
        Ok(ct) => ct,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => {
                eprintln!(
                    "{}{} cannot find the file specified",
                    "error".red().bold(),
                    ":".bold()
                );
                std::process::exit(2)
            }
            other_error => panic!("{:?}", other_error),
        },
    };

    let result = match args.algorithm.as_str() {
        "md5" => util::compute_md5(file),
        "sha128" => util::compute_sha128(file),
        "sha224" => util::compute_sha224(file),
        "sha256" => util::compute_sha256(file),
        "sha384" => util::compute_sha384(file),
        "sha512" => util::compute_sha512(file),

        // It's impossible to get to this arm. But if you do, please make an issue
        _ => panic!("unknown algorithm {:?}", args.algorithm),
    };

    let output = if args.unformatted {
        format!("{}", result)
    } else {
        format!(
            "The {} hash of {:?} is {}",
            args.algorithm.to_ascii_uppercase(),
            args.path,
            result
        )
    };
    println!("{}", output)
}
