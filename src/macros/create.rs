/// Convenience macro for put with ConditionExpression parameter to ensure
/// records are only “created” and not overwritten when inserting new records
/// into the table.
/// 
/// # Examples
/// 
/// ```
/// create!(
///     client;
///     Task {
///         project: "foo_project".to_string(),
///         employee: "e42069".to_string(),
///         description: "nothin' but chillin' 20's".to_string(),
///         some_metadata: "baz".to_string(),
///         ..Default::default()
///     }
/// )?;
/// ```
#[macro_export]
macro_rules! create {
    ($client:expr; $inst:expr) => {{
        let inst = $inst;
        let inst_keys = inst.primary_keys();

        $client
            .put_item()
            .table_name(inst.table__name())
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), inst_keys.hash.field()),
                ("#sk".to_string(), inst_keys.range.field()),
            ])))
            .set_item(Some(inst.into()))
            .send()
            .await
    }};
}
