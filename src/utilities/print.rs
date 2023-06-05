use std::fmt::Display;

pub fn print_arr(arr: Vec<impl Display>) {
    for item in arr {
        print_str(item);
    }
}
pub fn print_str(text: impl Display) {
    let data: String = text.to_string();
    println!("{}", data);
}
