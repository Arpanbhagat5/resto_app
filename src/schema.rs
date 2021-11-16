table! {
    item_status (status_id) {
        status_id -> Int8,
        status -> Nullable<Varchar>,
    }
}

table! {
    items (item_id) {
        item_id -> Int8,
        item_name -> Varchar,
    }
}

table! {
    table_order_items (id) {
        id -> Int8,
        item_id -> Int8,
        order_id -> Int8,
        item_status_id -> Int8,
        prep_time -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    table_orders (order_id) {
        order_id -> Int8,
        table_id -> Int8,
        created_at -> Timestamptz,
    }
}

table! {
    tables (table_id) {
        table_id -> Int8,
        is_free -> Bool,
    }
}

joinable!(table_order_items -> item_status (item_status_id));
joinable!(table_order_items -> items (item_id));
joinable!(table_order_items -> table_orders (order_id));
joinable!(table_orders -> tables (table_id));

allow_tables_to_appear_in_same_query!(
    item_status,
    items,
    table_order_items,
    table_orders,
    tables,
);
