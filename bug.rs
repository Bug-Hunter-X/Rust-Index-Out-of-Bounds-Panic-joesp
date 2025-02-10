fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);
    let index = 10;
    // This will panic at runtime
    println!("Value at index {}: {}", index, vec[index]);
}