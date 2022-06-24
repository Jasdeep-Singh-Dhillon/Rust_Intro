// Vectors - resizable arrays
use std::mem;

pub fn run() {
    let mut nums: Vec<i32> = vec![1,2,3,4];

    // Re-assign value
    nums[2] = 20;

    // Add on to vector
    nums.push(5);
    nums.push(6);

    // Pop off last value
    nums.pop();

    println!("{:?}", nums);

    // Get single value
    println!("Single Value: {}", nums[0]);

    // Get array length
    println!("Vector Length: {}", nums.len());

    // Arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&nums));
    println!("Integer occupies {} bytes", mem::size_of_val(&nums[0]));

    // Get slice
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in nums.iter() {
        println!("{}", x);
    }

    // Loop & mutate values
    for x in nums.iter_mut() {
        *x *= 2;
    }

    print!("Numbers Vec: {:?}",nums);
}