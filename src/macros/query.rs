#[macro_export]
macro_rules! query {
    (
        $client:expr;
        $table_name:expr;
        $key_cond_expr:expr;
        $($x:expr => $y:expr),+;
        $($a:expr => $b:expr),+
    ) => {{
        $client
            .query()
            .table_name($table_name)
            .key_condition_expression($key_cond_expr)
            .set_expression_attribute_names(Some(HashMap::from([
                $(( $x.to_string(), $y )),+
            ])))
            .set_expression_attribute_values(Some(HashMap::from([
                $(( $a.to_string(), $b )),+
            ])))
    }};
}

#[macro_export]
macro_rules! with_index {
    ($builder:expr, $index_name:expr) => {{
        $builder.index_name($index_name)
    }};
}

#[macro_export]
macro_rules! with_filter {
    ($builder:expr, $filter_expr:expr) => {{
        $builder.filter_expression($filter_expr)
    }};
}

#[macro_export]
macro_rules! exec {
    ($builder:expr => $items:ident) => {{
        let q = $builder.send().await?;
        $items::from(q.items().unwrap()).items()
    }};
}
