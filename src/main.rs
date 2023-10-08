fn main() {
    let mut x: i32 = 100;
    while x != 1 {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
    }
    println!("x: {x}");
}
