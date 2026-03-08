#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ( $( $k:expr => $v:expr),+ $(,)? ) => {
        // Using iterator instead of single inserts allows the HashMap to be allocated with the right capacity
        [$( ($k, $v), )*].into_iter().collect::<::std::collections::HashMap<_, _>>()
    };
}
