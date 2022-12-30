mod binarychunk;

fn main() {
    let a = 2;
    let b = f32::from_bits(a);
    println!("{} {}", a, b);
}
