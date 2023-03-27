// Vectors in Rust are similar to arrays, but they can grow or shrink in size.
// Vectors are declared with the vec! macro.
// Vectors are allocated on the heap, so they are slower than arrays.
// Vectors are also more flexible than arrays.

pub fn run(){
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Re-assign value
    numbers[2] = 20;
    // Add on to vector
    numbers.push(5);
    numbers.push(6);
    // Pop off last value
    numbers.pop();
    println!("{:?}", numbers);
    // Get single value
    println!("Single Value: {}", numbers[0]);
    // Get vector length
    println!("Vector Length: {}", numbers.len());
    // Vectors are heap allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));
    // Get Slice
    let slice: &[i32] = &numbers[0..2]; // Last index is not included
    println!("Slice: {:?}", slice);
    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);

    // Okay, vectors are like the true arrays in JS, but they are not the same.
    // Vectors are dynamic, and can grow and shrink in size.

    // Vectors can also be declared with the new() method.
    let mut numbers2: Vec<i32> = Vec::new();
    numbers2.push(1);
    numbers2.push(2);
    numbers2.push(3);
    numbers2.push(4);
    numbers2.push(5);
    println!("Numbers2 Vec: {:?}", numbers2);
    // Allocate all at once
    let numbers3: Vec<i32> = vec![0; 10];
    println!("Numbers3 Vec: {:?}", numbers3);
}