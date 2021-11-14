
// GET
#[derive(serde::Serialize)]
pub struct Table {
    pub table_id: i64,
    pub is_free: bool,
    pub order_id: i64,
}

// GET
#[derive(serde::Serialize)]
pub struct TableOrders {
    pub order_id: i64,
    pub table_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// POST
#[derive(serde::Serialize)]
pub struct NewTableOrder {
    pub table_id: i64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// GET
#[derive(serde::Serialize)]
pub struct TableOrderItems {
    pub item_id: i64,
    pub table_id: i64,
    pub item_status: ItemStatus,
    pub prep_time: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// POST
#[derive(serde::Serialize)]
pub struct NewTableOrderItem {
    pub table_id: i64,
    pub item_status: ItemStatus,
    pub prep_time: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Static data
#[derive(serde::Serialize, serde::Deserialize)]
pub enum ItemStatus {
    Preparing,
    Served,
    Canceled,
}
