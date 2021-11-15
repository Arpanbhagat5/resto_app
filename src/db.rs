use diesel::prelude::*;
use diesel::PgConnection;
use diesel::r2d2;


use crate::StdErr;
use crate::models::*;
use crate::schema::*;

use diesel::sql_types::*;
use std::env;

type PgPool = r2d2::Pool<r2d2::ConnectionManager<PgConnection>>;

pub struct Db {
    pool: PgPool,
}

#[derive(diesel::QueryableByName, Clone, Debug)]
pub struct IntOrderID {
    #[sql_type = "BigInt"]
    pub order_id: i64
}

#[derive(diesel::QueryableByName, Debug)]
pub struct BoolTableStatus {
    #[sql_type = "Bool"]
    pub is_free: bool
}

#[derive(diesel::QueryableByName, Debug)]
pub struct EnumTableOrderItemStatus {
    #[sql_type = "Item_status_enum"]
    pub item_status: ItemStatus
}

impl Db {
    pub fn connect() -> Result<Self, StdErr> {
        let db_url = env::var("DATABASE_URL")?;
        let manager = r2d2::ConnectionManager::new(db_url);
        let pool = r2d2::Pool::new(manager)?;
        Ok(Db { pool })
    }

    pub fn is_table_free(&self, table_id: i64 ) -> Result<Vec<BoolTableStatus>, StdErr> {
        let conn = self.pool.get()?;
        let result = diesel::sql_query("SELECT is_free FROM tables WHERE table_id=$1")
            .bind::<BigInt, _>(table_id)
            .load(&conn)?;
        Ok(result)
    }

    pub fn set_table_occupied(&self, table_id: i64, is_free: bool) -> Result<Tables, StdErr> {
        let conn = self.pool.get()?;
        let result = diesel::update(tables::table.filter(tables::table_id.eq(table_id)))
            .set(tables::is_free.eq(is_free))
            .get_result(&conn)?; // return updated tables as a result
        Ok(result)
    }

    pub fn create_table_order(&self, new_order: NewTableOrder) -> Result<TableOrders, StdErr> {
        let conn = self.pool.get()?;
        let order = diesel::insert_into(table_orders::table)
            .values(new_order)
            .get_result(&conn)?;
        Ok(order)
    }

    pub fn add_item_to_table_order(&self, new_order_item: NewTableOrderItem) -> Result<TableOrderItems, StdErr> {
        let conn = self.pool.get()?;
        let order_item = diesel::insert_into(table_order_items::table)
            .values(new_order_item)
            .get_result(&conn)?;
        Ok(order_item)
    }


    pub fn get_table_order_item_status(&self, order_id: i64, item_id: i64 ) -> Result<Vec<EnumTableOrderItemStatus>, StdErr> {
        let conn = self.pool.get()?;
        let result = diesel::sql_query("SELECT item_status FROM table_order_items WHERE order_id=$1 and item_id=$2")
            .bind::<BigInt, _>(order_id)
            .bind::<BigInt, _>(item_id)
            .load(&conn)?;
        Ok(result)
    }


    pub fn cancel_item_from_table_order(&self, order_id: i64, item_id: i64) -> Result<TableOrderItems, StdErr> {
        let conn = self.pool.get()?;
        let result = diesel::update(table_order_items::table
                .filter(table_order_items::order_id.eq(order_id))
                .filter(table_order_items::item_id.eq(item_id)))
            .set(table_order_items::item_status.eq(ItemStatus::Canceled))
            .get_result(&conn)?;
        Ok(result)
    }

    // pub fn get_item_id_from_table_id(&self, table_id: i64) -> Result<Vec<IntOrderID>, StdErr> {
    //     let conn = self.pool.get()?;
    //     let result = diesel::sql_query("SELECT order_id FROM table_orders WHERE table_id=$1")
    //         .bind::<BigInt, _>(table_id)
    //         .load(&conn)?;
    //     Ok(result)
    // }

    pub fn get_order_id_from_table_id(&self, table_id: i64) -> Result<Vec<IntOrderID>, StdErr> {
        let conn = self.pool.get()?;
        let result = diesel::sql_query("SELECT order_id FROM table_orders WHERE table_id=$1")
            .bind::<BigInt, _>(table_id)
            .load(&conn)?;
        Ok(result)
    }

    pub fn get_all_table_order_items(&self, order_id: i64) -> Result<Vec<TableOrderItems>, StdErr> {
        let conn = self.pool.get()?;
        let result = table_order_items::table
            .filter(table_order_items::order_id.eq(order_id))
            .load(&conn)?;
        Ok(result)
    }

    pub fn get_remaining_table_order_items(&self, order_id: i64) -> Result<Vec<TableOrderItems>, StdErr> {
        let conn = self.pool.get()?;
        let result = table_order_items::table
            .filter(table_order_items::order_id.eq(order_id))
            .filter(table_order_items::item_status.eq(ItemStatus::Preparing)) // from models
            .load(&conn)?;
        Ok(result)
    }
}