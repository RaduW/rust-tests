fn main() {
    let msg = get_message();

    println!("The message is {}.", msg);
}

fn get_message() -> String {
    let x = "abc";

    let msg = match x {
        "t" => "the message is T",
        "abc" => "the message is ABC",
        _ => "something else",
    };

    let _msg2 = format!("the message {} ", "fsdf");

    msg.into()
}
//
//fn flatten<T>(x: Option<Option<T>>) -> Option<T> {
//    match x {
//        Some(Some(x)) => flatten(Some(x)),
//        _ => None,
//    }
//}
