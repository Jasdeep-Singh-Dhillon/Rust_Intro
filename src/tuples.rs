// Max 12 elements
// Tuples group together values of different types

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 21);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}