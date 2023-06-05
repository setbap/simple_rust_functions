pub fn calc_median(mut list: Vec<f32>) -> Option<f32> {
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = list.len();
    if len == 0 {
        return None;
    } else if len % 2 == 0 {
        let upper_bond = list[len / 2];
        let lower_bond = list[(len - 1) / 2];
        return Some((upper_bond + lower_bond) / 2.0);
    } else {
        return Some(list[len / 2]);
    }
}
