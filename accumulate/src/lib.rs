pub fn map_function<T, S>(items: Vec<T>, f: &Fn(T) -> S) -> Vec<S> {
    map_closure(items, f)
}

pub fn map_closure<T, S, F>(items: Vec<T>, mut f: F) -> Vec<S>
    where F: FnMut(T) -> S {
    let mut result = Vec::new();
    for item in items {
        result.push(f(item));
    }
    result
}
