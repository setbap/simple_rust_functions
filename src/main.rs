use crate::utilities::*;

pub mod utilities;
fn main() {
    println!("Hello, world!");
    println!(" Median : {:?} ", calc_median(vec![9.0, 3.0, 4.2, 5.0]));
    println!(" Median : {:?} ", calc_median(vec![12.0, 2.0, 4.0,]));
    println!(" Median : {:?} ", calc_median(vec![]));
    println!(
        " Unique : {:?} ",
        find_unique_item(vec![1, 2, 3, 4, 4, 5, 2, 9, 10, 10, 1])
    );
    print_str(23);
    let mut arr = vec!["sina", "Aba", "Taghi", "zeyo", "ada", "taGHo"];
    sort_str(&mut arr);
    print_arr(arr);
}
