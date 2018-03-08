#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr,)+) => { hashmap!($($k => $v),+) };
    ($($k:expr => $v:expr),*) => {
        {
            let mut hashmap = HashMap::new();
            $(hashmap.insert($k, $v);)*
            hashmap
        }
    };
}
