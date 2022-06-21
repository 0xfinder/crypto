use sha256;

fn main() {
    let num = 69;
    println!("Hello, world {} + 1 = {}!", num, sha256::sha256(num));
}
