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
