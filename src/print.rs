pub fn run() {
    // Print to console
    println!("Hello from the print rs file");

    // Basic formatting
    println!("{} is from {}", "Jasdeep", "Delhi");

    //  Positional Formatting
    println!("{0} is from {1} and {0} likes to {2}", "Jasdeep", "Delhi", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name="Jasdeep", activity="Apex");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic Math
    println!("10 + 10 = {}", 10+10);
}