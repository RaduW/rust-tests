#[macro_use]

mod pii;

fn main() {
    println!("Hello, world! ");

    let a = 33;
    let b = a as f64 / 1000.0;
    println!("b is {}", b);
    //pii::display_pii();
    test_iterate_through_array();
}

fn test_iterate_through_array() {
    let x = [1, 2, 3];

    for v in &x {
        println!("index is {}", v);
    }
}
