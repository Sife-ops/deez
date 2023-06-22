#[macro_export]
macro_rules! create {
    ($client:expr; $inst:expr) => {{
        let inst = $inst;

        $client
            .put_item()
            .table_name(inst.table__name())
            .condition_expression("attribute_not_exists(#pk) AND attribute_not_exists(#sk)")
            .set_expression_attribute_names(Some(HashMap::from([
                (
                    "#pk".to_string(),
                    inst.index_key(Index::Primary, Key::Hash).field,
                ),
                (
                    "#sk".to_string(),
                    inst.index_key(Index::Primary, Key::Range).field,
                ),
            ])))
            .set_item(Some(inst.into()))
            .send()
            .await?
    }};
}

