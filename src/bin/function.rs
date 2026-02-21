fn main (){
println!("i am inside of main");
    outside_fn(123, 'h')
}

fn outside_fn(x:i32, y:char){
    println!("multiple parameter call is: {x}{y} ");
    // we love shadows,
    let y = {
        let x = 3;
        x+1
    };// now this block became a statement,
    //returned value of x+1
    println!("{y}");
    println!("value {}", let_there_be_another_fn())
}

//  return tipini -> ile belirtmen gerekiyor......
fn let_there_be_another_fn ()-> i32{
    5
}
// there is a long lecture about statement and
// expression, which ive learned it before hardway../

// {
// let x = 3;
// x + 1
// }    this block is expression