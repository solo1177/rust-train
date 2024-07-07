pub fn print() {
    println!("sample!");
}

pub fn value_print() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}

pub fn if_test() {
  let number = 1;
    if number != 0 {
        println!("number was something other than zero");
    }
}
