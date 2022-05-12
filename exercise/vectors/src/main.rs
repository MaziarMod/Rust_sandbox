//Vectors are resizable arrays

fn main() {
    
    let mut my_vector: Vec<i8> = vec![3,2,5,6,7];

    // Get all the values
    println!("{:?}", my_vector);

    // Get single value
    println!("Single Value: {}", my_vector[2]);

    //Reassign a value
    my_vector[2] = 100;
    println!("reassign : {}", my_vector[2]);

    // Get length
    println!("Length : {}", my_vector.len());

    // Add an element to the vector
    my_vector.push(23);
    my_vector.push(111);
    
    // pop last value
    my_vector.pop();

    //Vector are stack allocated
    println!("Vector takes up {} bytes", std::mem::size_of_val(&my_vector));

    // Get Slice
    let slice1: &[i8] = &my_vector;
    println!("Slice 1: {:?}", slice1);

    let slice2: &[i8] = &my_vector[0..=2];
    println!("Slice 2: {:?}", slice2);

    //Loop
    for el in my_vector.iter() {
        println!("immut=> {}", el);
    }

    //Loop and mutate values
    for el in my_vector.iter_mut() {
       *el += 20;
    }
    println!("Vector: {:?}", my_vector);
}
