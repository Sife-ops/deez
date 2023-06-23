#[macro_export]
macro_rules! vec_from_query {
    ($q:expr => $i:ident) => {{
        $i::from($q.items().unwrap()).items()
    }};
}
