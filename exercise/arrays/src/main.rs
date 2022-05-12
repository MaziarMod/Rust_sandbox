//Arrays  - Fixed list where elements are the same data types

fn main() {
    //let my_array: [i8; 5] = [3,2,5,6]; cause an error because length of array is less than 5
    //let my_array: [i8; 5] = [3,2,5,6,"100"]; cause an error because one of the elements is string

    let mut my_array: [i8; 5] = [3,2,5,6,7];

    // Get all the values
    println!("{:?}", my_array);

    // Get single value
    println!("Single Value: {}", my_array[2]);

    //Reassign a value
    my_array[2] = 100;
    println!("reassign : {}", my_array[2]);

    // Get length
    println!("Length : {}", my_array.len());

    //Arrays are stack allocated
    println!("Array takes up {} bytes", std::mem::size_of_val(&my_array));

    // Get Slice
    let slice1: &[i8] = &my_array;
    println!("Slice 1: {:?}", slice1);

    let slice2: &[i8] = &my_array[0..2];
    println!("Slice 2: {:?}", slice2);
}
