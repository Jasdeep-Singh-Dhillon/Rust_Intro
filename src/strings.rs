// Primitive str = Immutable fixed length string somewhere in memory
// String = Growable, heap allocated data structure - Use when you need to modify or own string data

pub fn run() {
    // Immutable
    let hello = "Hello";

    // Growable
    let mut hello_w = String::from("Hello World");

    // Get length
    println!("Length: {}", hello.len());

    // Push Char
    hello_w.push('!');

    // Push String
    hello_w.push_str(" A lengthy string");

    // Capacity in bytes
    println!("Capacity: {}", hello_w.capacity());

    // Check if empty
    println!("Is Empty: {}", hello_w.is_empty());

    // Contains
    println!("Contains 'World' {}", hello_w.contains("Worl"));

    // Replace
    println!("Replace: {}", hello_w.replace("World", "Jasdeep"));

    // Loop through string by whitespace
    for word in hello_w.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, hello_w));
}