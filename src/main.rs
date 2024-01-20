use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(help = "Input text", required = true)]
    text: Vec<String>,

    #[arg(short = 'n', help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let args = Args::parse();

    println!("{:#?}", args);
}
