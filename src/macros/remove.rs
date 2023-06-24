#[macro_export]
macro_rules! remove {
    (
        $client:expr;
        $ent:expr
    ) => {{
        let ent = $ent;
        let keys = ent.primary_keys();

        $client
            .delete_item()
            .table_name(ent.table__name())
            .condition_expression("attribute_exists(#pk) AND attribute_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                ("#pk".to_string(), keys.hash.field()),
                ("#sk".to_string(), keys.range.field()),
            ])))
            .set_key(Some(HashMap::from([
                (keys.hash.field(), keys.hash.av()),
                (keys.range.field(), keys.range.av()),
            ])))
            .send()
            .await
    }};
}
