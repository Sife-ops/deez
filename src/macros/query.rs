/// Convenience macro for converting a query result back to `Vec<T>`.
/// 
/// # Examples
/// 
/// ```
/// let tasks = vec_from_query!(
///     client
///         .query()
///         .table_name(Task::table_name())
///         .key_condition_expression("#pk = :pk and begins_with(#sk, :sk)")
///         .set_expression_attribute_names(Some(HashMap::from([
///             ("#pk".to_string(), keys.hash.field()),
///             ("#sk".to_string(), keys.range.field()),
///         ])))
///         .set_expression_attribute_values(Some(HashMap::from([
///             (":pk".to_string(), keys.hash.av()),
///             (":sk".to_string(), keys.range.av()),
///         ])))
///         .send()
///         .await?
/// 
///     => TaskItems
/// );
/// ```
#[macro_export]
macro_rules! vec_from_query {
    ($q:expr => $i:ident) => {{
        $i::from($q.items().unwrap()).items()
    }};
}
