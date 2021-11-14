table! {
    use diesel::sql_types::*;
    use crate::models::Item_status_enum;

    items (item_id) {
        item_id -> Int8,
        item_name -> Varchar,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Item_status_enum;

    table_order_items (id) {
        id -> Int8,
        item_id -> Int8,
        table_id -> Int8,
        item_status -> Item_status_enum,
        prep_time -> Int4,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Item_status_enum;

    table_orders (order_id) {
        order_id -> Int8,
        table_id -> Int8,
        created_at -> Timestamptz,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Item_status_enum;

    tables (table_id) {
        table_id -> Int8,
        is_free -> Bool,
    }
}

joinable!(table_order_items -> items (item_id));
joinable!(table_order_items -> tables (table_id));
joinable!(table_orders -> tables (table_id));

allow_tables_to_appear_in_same_query!(
    items,
    table_order_items,
    table_orders,
    tables,
);
