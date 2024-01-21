use text_io::read;

fn main() {
    println!("How much do you want to bet?");

    let bet: i32 = read!();
    println!("You bet ${}", bet)
}
