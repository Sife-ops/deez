#[macro_export]
macro_rules! batch_write {
    (
        $client:expr;
        writes: $( $w:expr ),*;
        deletes: $( $d:expr ),*
    ) => {{
        let mut m: HashMap<String, Vec<WriteRequest>> = HashMap::new();

        $({
            let w = $w;
            let table_name = w.table__name();
            let wr = WriteRequest::builder()
                .put_request(PutRequest::builder().set_item(Some(w.into())).build())
                .build();
            if let Some(x) = m.get_mut(&table_name) {
                x.push(wr);
            } else {
                m.insert(table_name, vec![wr]);
            }
        })*

        $({
            let d = $d;
            let keys = d.primary_keys();
            let dr = WriteRequest::builder()
                .delete_request(
                    DeleteRequest::builder()
                        .key(keys.hash.field(), keys.hash.av())
                        .key(keys.range.field(), keys.range.av())
                        .build(),
                )
                .build();
            if let Some(x) = m.get_mut(&d.table__name()) {
                x.push(dr);
            } else {
                m.insert(d.table__name(), vec![dr]);
            }
        })*

        $client
            .batch_write_item()
            .set_request_items(Some(m))
            .send()
            .await
    }};
}
