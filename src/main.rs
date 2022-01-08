fn main() {
    let message = std::env::args().nth(1).expect("Usage: catsay \"[message]\"");
    println!("{}", message);
    println!(" \\");
    println!("  \\");
    println!("    ^_^");
    println!("  ( ° ° )");
    println!("   =(I)=");
}
