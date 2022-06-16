use clap::Parser;

mod cli_args;
mod parser;

fn main() {
    let args = cli_args::Args::parse();
    println!("{}", parser::run(args));
}
