use structopt::StructOpt;
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
}
fn main() {
    let opt = CatsayArgs::from_args();
    println!("{:?}", opt);
    let eye = match opt.dead {
        true => 'x',
        false => '°'
    };
    println!("{}", opt.message);
    println!(" \\");
    println!("  \\");
    println!("    ^_^");
    println!("  ( {} {} )", eye, eye);
    println!("   =(I)=");
}
