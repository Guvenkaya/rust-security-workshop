fn main() {
    let mut size: usize = 1024; // Start with 1KB
    while let Some(double_size) = size.checked_mul(2) {
        match Vec::<usize>::with_capacity(double_size) {
            _ => println!("Successfully allocated {} bytes", size),
        }
        size = double_size; // Double the allocation size on each iteration
    }

    println!("Failed to allocate memory or reached maximum allocation size.");
}
