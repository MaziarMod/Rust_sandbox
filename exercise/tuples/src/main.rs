// Tuples group values of different type
// Max 12 elements

fn main() {

    let todo_list: (&str, &str, bool, i8) = ("Reading a Book", "Data structure and Algorithm", true, 1);
    println!("Task: {} ,Note: {}, Is Completed: {}, Priority: {}", todo_list.0, todo_list.1, todo_list.2, todo_list.3);
    
}
