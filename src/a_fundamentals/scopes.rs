pub fn scopes() {
    let x = 5;
    {
        let y = 19;
        print!("{}, {}", x, y);
    }
    // Generates error
    // println!("{}, {}", x, y);

}