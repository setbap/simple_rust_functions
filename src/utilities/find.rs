use std::{collections::HashMap, hash::Hash};

pub fn find_unique_item<T>(list: Vec<T>) -> Vec<T>
where
    T: Eq + Hash + Copy,
{
    let mut hash_map: HashMap<_, i32> = HashMap::new();
    for item in list.iter() {
        let count = *hash_map.get(item).unwrap_or(&0);
        hash_map.insert(item, count + 1);
        // hash_map
        //     .entry(str_item)
        //     .and_modify(|e| *e += 1)
        //     .or_insert(1);
    }

    hash_map
        .into_iter()
        .filter(|(_, count)| *count == 1)
        .map(|(&x, _)| x)
        .collect()
}
