use std::cmp::Ordering;
use std::io;
//
use rand::Rng;
//
pub fn deneme() {}
fn main() {
    println!("Make a guess... Please input a number between 1-1000");
    let secret_number = rand::thread_rng().gen_range(1..=1000);
    loop {
        // println!("The secret number is: {secret_number}");
        //its for heap, not for stack
        let mut guess = String::new();
        // std::io::stdin().read_line(&mut guess)
        io::stdin().read_line(&mut guess)
            .expect("there was an error");
        //  there shall be shadowing , in the cave...
        let guess:usize = match guess.trim().parse() {
            Ok(num)=> num,
            Err(_) => {println!("dogru yazsana aq");
                continue }
        };
        // u32 i32 vs u64 i64 ......
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal=> {
                println!("😁    you win ");
                break;
            }

        }
        // println!("your guess is {}", guess);
        print!("\u{0007}");
    }
    
}




