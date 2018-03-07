#[macro_export]
macro_rules! hashmap {
    ($($k:expr => $v:expr),*) => {
        {
            let mut hashmap = HashMap::new();
            $(hashmap.insert($k, $v);)*
            hashmap
        }
    };
}

// ($elem:expr;$n:expr) => ($crate::vec::from_elem($elem,$n));
// ($($x:expr),*) => (
//     <[_]>::into_vec($crate::boxed::Box::new([$($x),*]))
// ); 
// ($($x:expr,)*) => (vec![$($x),*])
