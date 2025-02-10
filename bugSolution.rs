fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    println!("Vector: {:?}", vec);
    let index = 1;
    // Safe access using 'get()'
    match vec.get(index) {
        Some(value) => println!("Value at index {}: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    };
    
    //Alternative using if let
    let index2 = 10;
    if let Some(value) = vec.get(index2) {
        println!("Value at index {}: {}", index2, value);
    } else {
        println!("Index {} is out of bounds", index2);
    }
} 