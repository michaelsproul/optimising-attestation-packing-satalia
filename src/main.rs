use clap::Parser;

use sigmaprime::approaches;
use sigmaprime::Instance;

#[derive(Parser, Debug)]
#[clap(name = "AAPP MIP solver")]
struct Args {
    /// Approach to run
    #[clap(short, long)]
    approach: String,

    /// Path for the input instance
    #[clap(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let instance = Instance::from_file(&args.input).unwrap();
    match &args.approach[..] {
        "mip_approach" => approaches::mip_approach(&instance),
        "n_unique_roots" => approaches::n_unique_roots(&instance),
        _ => {
            println!("valid approach not specified.")
        }
    }
}
