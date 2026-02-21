use std::io;
fn main() {
    let a = [1,2,3,4,5,6,7];
    println!("please enter an arrayy index");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("failed to read the line");

    let index:usize = index.trim().parse().expect("index is wrong number");

    let element = a[index];

    println!("the value of the element at index {index} is {element}")

}