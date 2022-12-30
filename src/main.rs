mod binarychunk;

fn main() {
    let a = [1, 2, 3];
    let b: &[u32] = &a;
    let c = b[2];
    println!("{}", c);
}
