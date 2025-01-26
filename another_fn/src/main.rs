fn main() {
    println!("Hello, world!");
    let mut num = 2;
    num = another_function(num);

    println!("{}", num);
}

fn another_function(x: i32) -> i32 {
    x + 1 // si se agrega ; falla
}