#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr), *) => [
        {
        	use std::collections::HashMap;
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*
            map
        }
    ]
}
