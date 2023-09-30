pub fn ownership_references(arg: String) {
    // This fancy stuff either gets the first argument as a String, or prints
    // usage and exits if an argument was not supplied to the program.

    // 1. Write a function `inspect` that takes a reference to a String, returns nothing, but
    // prints whether the contents of the String is plural or singular. Then uncomment and run this
    // code with `cargo run apple` and `cargo run apples'.  Hint: use `.ends_with("s")` on the
    // String reference
    //

    let mut the_args = arg.clone();

    inspect(&the_args);

    // 2. Write a function `change` that takes a *mutable* reference to a String and adds an "s" to
    // the String if it doesn't already end with "s". Then uncomment and run the code below with
    // `cargo run apple`.  Hint: use `.push_str("s")` on the mutable String reference to add an "s".
    //
    change(&mut the_args);
    println!("I have many {}", the_args);

    // 3. Write a function `eat` that accepts ownership of (consumes) a String and returns a bool
    // indicating whether or not the String both starts with a "b" AND contains an "a".
    // Hint 1: use `.starts_with("b")` and `.contains("a")`
    // Hint 2: `&&` is the boolean "AND" operator
    //
    if eat(the_args) {
       println!("Might be bananas");
    } else {
       println!("Not bananas");
    }

    // Try running this program with "boat", "banana", and "grapes" as the arguments :-)

    // Challenge: Write a function "bedazzle" that takes a mutable reference to a String and
    // ignores what is in the string and replaces the contents of the string with the String
    // "sparkly". Then uncomment the code below.
    //
    // Hint: You will need to dereference the mutable reference in order to assign it a
    // new value.
    //
    // let mut material = "mud".to_string();
    // println!("This material is just `{}`.", material);
    // bedazzle(&mut material);
    // println!("Wow! Now the material is `{}`!", material);
}

fn inspect(s: &String) {
    if s.ends_with("s"){
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s")
    }
}

fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("s") {
        return true;
    } else {
        return false;
    }
}