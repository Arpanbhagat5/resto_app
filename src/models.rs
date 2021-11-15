// bring all schema models to scope here
use crate::schema::*;


// GET
#[derive(serde::Serialize)]
pub struct Table {
    pub table_id: i64,
    pub is_free: bool,
}

// GET
#[derive(serde::Serialize, diesel::Queryable)]
pub struct TableOrders {
    pub order_id: i64,
    pub table_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// POST
#[derive(Debug, serde::Deserialize, diesel::Insertable)]
#[table_name = "table_orders"]
pub struct NewTableOrder {
    pub table_id: i64,
}

// GET
#[derive(serde::Serialize, diesel::Queryable)]
pub struct TableOrderItems {
    pub id: i64,
    pub item_id: i64,
    pub order_id: i64,
    pub item_status: ItemStatus,
    pub prep_time: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// POST
#[derive(serde::Deserialize, diesel::Insertable)]
#[table_name = "table_order_items"]
pub struct NewTableOrderItem {
    pub item_id: i64,
    pub order_id: i64,
    pub item_status: ItemStatus,
    pub prep_time: i32,
}

// Static data
#[derive(Debug, serde::Serialize, serde::Deserialize, diesel_derive_enum::DbEnum)]
#[DieselType = "Item_status_enum"]
pub enum ItemStatus {
    Preparing,
    Served,
    Canceled,
}
