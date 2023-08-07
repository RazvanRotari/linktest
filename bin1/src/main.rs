use lib2::add;
use lib2::tokio;

fn main() {
    add(1, 2);
    let d = tokio::time::Instant::now();
    println!("Hello, world!");
}
