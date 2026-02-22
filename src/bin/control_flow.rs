fn main() {
    // fn explode (){
    //     let arr = [0;100000000];
    // }
    // explode();
    //if is a expression, difference from js
    some_func();
    // branches
    label_loop();
    if_let();
    loop_yyyy();
    while_loop();
    element_of_collection();
    for_loop();
    countdown();

     fn some_func(){
        let number = 3;
        if number < 5 {
            println!("condition was true")
        } else {
            println!("condition was false")
        }
    }

}
//Unlike languages such as Ruby and JavaScript,
// Rust will not automatically try to convert non-Boolean types
// to a Boolean.

// else if kullanimi da var, neyini not aliyorsam aq
//--------------------------------------------------

fn if_let (){
    let condition = true;
    let number = if condition {5} else {6};
    println!("the value of number is :{number}")
}
// 3 kullanim var
// loop, while, for
fn loop_yyyy (){

    let mut num = 1;

    let last_num = loop{
        println!("i am looping isnt that fun!?");
         num = num + 1;
        print!("\u{0007}"); // :) isnt that fun!?
        if num == 100 {break num;}
        // bir de break deger dondurebiliyor,
        //artik nasil alisirim bilmiyorum.


    };
    // that was returning a value from loop
    println!("this times loop count : {}", last_num-1);


}
fn label_loop (){
    let mut outer_count = 0;
    // as you see this is a label
    //'outside_loop
    'outside_loop: loop {
        outer_count = outer_count+1;
        println!("outer count is :{outer_count}");
        if outer_count == 100 { break }
        let mut inner_count = 0;
        loop {
            inner_count = inner_count+1;
            println!("inner count is :{inner_count}");
            if inner_count == 10 {break 'outside_loop}
        }

    }
}

fn while_loop () {
    let mut count:u32 = 10;
    println!("start the countdown, 10 minus");
    while count !=0 {
        print!("{count} ");
        // count -=1  // THAT SEEMS IMPORTANT
        count -=1;
    }
    println!("LIFTOFF!!!")
}
// burada daha net bir ornek var
// let x = {
// count -= 1    // x = ()
// };
//
// let y = {
// count -= 1;   // y = ()
// };

//while ile array vb , elemnet of collection loop yapmak


// book says this approach is error prone, use for
// and says for is also performant
fn element_of_collection (){
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
        while index< a.len() {
            println!("the value of iteration : {}", a[index]);
                index +=1;
        }
}
// this code is more efficent says the book
fn for_loop (){
    let array = [10,20,30,40,50];
    for element in array{
        println!("the value of the element is {element}")
    }
}

//countdown for loop hali

fn countdown (){
    for number in (1..1000).rev() {
        print!("{number} ")
    }
    println!("LIFTOFFFFFsdfsdfsdfsdf !!!")
}