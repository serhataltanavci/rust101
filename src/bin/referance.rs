fn main() {
    another_way();
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    //here we are transfering value of s1 to s2
    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)// look here is a tuple
}

fn another_way (){
    let s1 = String::from("Helloooo");
    let len = calculate_another_length(&s1);
    println!("The length of '{s1}' is {len}....")
}
fn calculate_another_length (s: &String)// &look& at the type carefully!
    ->usize{ s.len() }
