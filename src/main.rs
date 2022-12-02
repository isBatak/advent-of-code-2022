use clap::Parser;
mod solutions;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {


   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   solution: u8,
}


fn main() {
    let args = Args::parse();

    match args.solution {
        1 => solutions::day1::run(),
        2 => solutions::day2::run(),
        _ => println!("No solution for day {}", args.solution),
    }
}
