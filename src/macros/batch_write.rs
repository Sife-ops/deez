/// Convenience macro for batch write operations.
/// 
/// # Examples
/// 
/// ```
/// batch_write!(
///     client;
///     writes:
///         Task {
///             task_id: Some("87cb64a9-6431-406f-89d8-e91cb7ea944b".to_string()),
///             project: Some("foo_project".to_string()),
///             employee: Some("Mark".to_string()),
///             ..Default::default()
///         };
///     deletes:
///         Task {
///             task_id: Some("d9bb6b19-e306-4705-9773-960abe6c5078".to_string()),
///             project: Some("foo_project".to_string()),
///             employee: Some("Jimbo".to_string()),
///             ..Default::default()
///         },
///         Task {
///             task_id: Some("8ddbddaf-2072-4da1-b7c4-04aa31006b41".to_string()),
///             project: Some("foo_project".to_string()),
///             employee: Some("Steve".to_string()),
///             ..Default::default()
///         }
/// )?;
/// ```
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
