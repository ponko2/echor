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
    let Args { text, omit_newline } = Args::parse();
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
