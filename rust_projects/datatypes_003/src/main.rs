fn main() {
    let mut x = 10;
    let mut y = 20;
    println!("Value of x = {x} and value of y = {y}", x = x, y = y);
    x = x ^ y;
    y = x ^ y;
    x = x ^ y;
    println!("Value of x = {x} and value of y = {y}", x = x, y = y);    
}
