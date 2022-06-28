fn main () {
    // ownership
    // let _s1 = String::from("hello");
    // let _s2 = _s1.clone();
    // println!("{}", _s1);

    let mut s = String::from("Hello");
    // take_ownership(s);
    // println!("{}", s);

    let length = calculate_length(&mut s);
    println!("Do dai cua {} la {}", s, length);
}

// fn take_ownership(some_string: String) {
//     println!("{}", some_string)
// }

fn calculate_length(some_string: &mut String) -> usize {
    some_string.push_str("World");
    let length = some_string.len();
    length
}