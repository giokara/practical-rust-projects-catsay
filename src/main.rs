use structopt::StructOpt;
use colored::*;
/// Let the cat speak for you in the shell
#[derive(StructOpt, Debug)]
#[structopt(name = "catsay")]
struct CatsayArgs {
    /// The message the cat should be saying
    #[structopt(short, long, default_value="Hello, world!")]
    message: String,
    /// Show the cat to be dead
    #[structopt(short, long)]
    dead: bool,
    /// Read the cat figure from a file
    #[structopt(short, long, parse(from_os_str))]
    file: Option<std::path::PathBuf>,
}
fn main() {
    let opt = CatsayArgs::from_args();
    if opt.message.to_lowercase().contains("woof") {
        eprintln!("A cat shouldn't bark!");
    }
    let mut cat = concat!(
        " \\\n",
        "  \\\n",
        "    ^_^\n",
        "  ( {eye} {eye} )\n",
        "   =(I)="
    ).to_string();
    let eye = match opt.dead {
        true => "x".red().bold(),
        false => "°".normal()
    };
    if opt.file.is_some() {
        cat = std::fs::read_to_string(opt.file.unwrap()).expect("Could not parse file");
    }
    println!("{}", opt.message.yellow().on_purple());
    // TODO fix dead eye formatting
    println!("{}", cat.replace("{eye}", &eye));
}
