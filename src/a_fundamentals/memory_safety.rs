pub fn memory_safety() {
    let enigma: i32;
    if true {
        enigma = 42
    } else {
        enigma = 7
    }

    println!("enigma is {}", enigma); // error
}