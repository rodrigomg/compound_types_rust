fn main() {
    tuple_sample();
    tuple_destructure();
    tuple_destructure_by_index();
}

fn tuple_sample(){
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);
}

fn tuple_destructure(){
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
}

fn tuple_destructure_by_index(){
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let one = x.0;
    let two = x.1;
    let last = x.2;
    println!("The value of one is: {}", one);
    println!("The value of two is: {}", two);
    println!("The value of last is: {}", last);
}
