// bring all schema models to scope here
use crate::schema::*;


/// Table entity in the restaurant
/// Used in GET request
#[derive(Queryable, serde::Serialize)]
pub struct Tables {
    pub table_id: i64,
    pub is_free: bool,
}

/// Table order entity for a table
/// There would be only one per table at a time
/// Used in GET request
#[derive(serde::Serialize, diesel::Queryable)]
pub struct TableOrders {
    pub order_id: i64,
    pub table_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Entity used for creating new orders for a table
/// Used in POS request
#[derive(Clone, Debug, serde::Deserialize, diesel::Insertable)]
#[table_name = "table_orders"]
pub struct NewTableOrder {
    pub table_id: i64,
}

/// Represents one of the item of the order of a table
/// Used in GET request
#[derive(serde::Serialize, diesel::Queryable)]
pub struct TableOrderItems {
    pub id: i64,
    pub item_id: i64,
    pub order_id: i64,
    pub item_status_id: i64,
    pub prep_time: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// This is used for adding new items to an order for a table
/// Used in POS request
#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "table_order_items"]
pub struct NewTableOrderItem {
    pub item_id: i64,
    pub order_id: i64,
    pub prep_time: i32,
}
