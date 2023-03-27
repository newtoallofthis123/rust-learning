// Arrays in rust are fixed length, and homogeneous. They are stack allocated.


pub fn run(){
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // Get single value
    println!("Single Value: {}", numbers[0]);
    // Get array length
    println!("Array Length: {}", numbers.len());
    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));
    // Get Slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}