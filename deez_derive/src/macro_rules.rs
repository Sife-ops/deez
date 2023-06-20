macro_rules! insert_index {
    ($map: ident, $name: expr, $hash: expr, $range: expr, $index: expr) => {
        $map.insert(
            $name,
            IndexKeys {
                index: $index,
                hash: IndexKey {
                    field: $hash,
                    ..Default::default()
                },
                range: IndexKey {
                    field: $range,
                    ..Default::default()
                },
            },
        );
    };
}

pub(crate) use insert_index;

macro_rules! insert_gsi {
    ($map: ident, $name: expr, $hash: expr, $range: expr, $index: expr) => {
        if let Some(g) = $name {
            insert_index!($map, g, $hash.unwrap(), $range.unwrap(), $index);
        }
    };
}

pub(crate) use insert_gsi;

macro_rules! compose_key {
    ($index_key: expr) => {{
        let mut composed = quote! {};
        for (i, _) in $index_key.composite.iter().enumerate() {
            let composite = $index_key.composite.iter().find(|c| c.position == i).unwrap();
            let ident = composite.syn_field.ident.as_ref().unwrap();
            let ident_string = ident.to_string();

            composed = quote! {
                #composed
                composed.push_str(&format!("#{}_{}", #ident_string, self.#ident));
            };
        }
        composed
    }};
}

pub(crate) use compose_key;
