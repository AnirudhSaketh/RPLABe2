fn main() {
    let x = 5;
    println!("Outside block, x = {}", x);

    {
        let x = x + 1;
        println!("Inside inner block, x = {}", x);

        let y = 10;
        println!("Inside inner block, y = {}", y);
    }

    println!("Outside block again, x = {}", x);

    let x = x * 2;
    println!("After shadowing outside block, x = {}", x);
}
