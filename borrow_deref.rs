fn main() {
    let x = 10;

    let y = &x;

    println!("Value of x is: {}", x);
    println!("Value of y (reference to x) is: {}", y);
    println!("Dereferenced y is: {}", *y);
}
