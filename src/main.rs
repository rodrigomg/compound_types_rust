fn main() {
    tuple_sample();
    tuple_destructure();
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
