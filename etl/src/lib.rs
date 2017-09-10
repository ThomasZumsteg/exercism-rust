use std::collections::BTreeMap;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();
    for (v, keys) in input.iter() {
        for k in keys.iter() {
            result.insert(k.to_lowercase().next().unwrap(), *v);
        }
    }
    result
}
