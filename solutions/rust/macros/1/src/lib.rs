#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };
    ( $( $k:expr => $v:expr),+ $(,)? ) => {
        [$( ($k, $v), )*].into_iter().collect::<::std::collections::HashMap<_, _>>()
    };
}
