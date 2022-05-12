// Reference pointers - point to a resource in memory

fn main() {
   // primitive arrays
   let array1 = [9,10,123];
   let array2 = array1;

   println!("Values of primitive arrays: {:?}", (array1, array2));

   // with non-primitives, if you assign another variable to a piece of data, the first variable will
   // no longer hold that value. You will need to use a reference (&) to point to the resource
   
   // vectors
   // let vector1 = vec![9,10,123];
   // let vector2 = vector1;

   // vector1 no longer hold the values after assigning to vector2, so the following line will throw an error
   //println!("Values of vectors: {:?}", (vector1, vector2));

   let vector1 = vec![9,10,123];
   let vector2 = &vector1;

   println!("Values of vectors: {:?}", (&vector1, vector2));
}
