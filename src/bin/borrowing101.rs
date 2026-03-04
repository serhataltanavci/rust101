fn main() {
    let mut s = String::from("lbla bla bla");
    // let r1 = &s;
    // println!("{r1} ");
    // let r2 = &mut s;
    // println!("{r2}");
    // let r3= &mut s;
    // println!("{r3}");
    // thanks to nonlexical lifetime...

    let bytes = s.as_bytes();
    println!("{:?}", bytes)}