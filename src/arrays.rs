// Arrays - Fixed list where elements are the same data types
use std::mem;

pub fn run() {
    let mut nums: [i32; 4] = [1,2,3,4];

    // Re-assign value
    nums[2] = 20;

    println!("{:?}", nums);

    // Get single value
    println!("Single Value: {}", nums[0]);

    // Get array length
    println!("Array Length: {}", nums.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&nums));
    println!("Array occupies {} bytes", mem::size_of_val(&nums[0]));

    // Get slice
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

}