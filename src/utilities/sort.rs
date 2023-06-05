pub fn sort_str(list: &mut Vec<&str>) {
    list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
}
