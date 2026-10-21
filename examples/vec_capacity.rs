//! Predict length and capacity before each push, then compare observations.
fn main() {
    let mut values = Vec::new();
    println!("length,capacity");
    println!("{},{}", values.len(), values.capacity());
    for value in 0..20 {
        values.push(value);
        println!("{},{}", values.len(), values.capacity());
    }
    println!("Does capacity always equal length? Is an exact growth factor guaranteed?");
}
