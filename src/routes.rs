// For enabling DB access
use rocket::State;
// For enabling Rust struct <-> JSON ser-de
use rocket_contrib::json::Json;

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

// order routes

// TODO: check and set table status before creating an order
#[rocket::post("/table/create_order", data = "<new_table_order>")]
fn create_table_order(db: State<Db>, new_table_order: Json<NewTableOrder>) -> Result<Json<TableOrders>, StdErr> {
    db.create_table_order(new_table_order.0).map(Json)
}

// some error related to responder for enum type. Ref: https://github.com/serde-rs/serde/issues/912
#[rocket::post("/table/add_item", data = "<new_order_item>")]
fn add_item_to_table_order(db: State<Db>, new_order_item: Json<NewTableOrderItem>) -> Result<Json<TableOrderItems>, StdErr> {
    db.add_item_to_table_order(new_order_item.0).map(Json)
}

// Enum issue
#[rocket::get("/table/<table_id>/all_items")]
fn get_all_table_order_items(db: State<Db>, table_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    db.get_all_table_order_items(response[0].order_id).map(Json)
}

// Enum issue
#[rocket::get("/table/<table_id>/remaining_items")]
fn get_remaining_table_order_items(db: State<Db>, table_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    db.get_remaining_table_order_items(response[0].order_id).map(Json)
}

// Enum issue, untested
#[rocket::get("/table/<table_id>/cancel/<item_id>")]
fn cancel_item_from_table_order(db: State<Db>, table_id: i64, item_id: i64) -> Result<Json<TableOrderItems>, StdErr> {
    let item_id : i64 = item_id;
    let item_id2 = item_id.clone();

    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let response2 = response.clone();

    db.get_table_order_item_status(response[0].order_id, item_id);
    db.cancel_item_from_table_order(response2[0].order_id, item_id2).map(Json)
}

// Pub function to return API routes; 'api/' needs to be used in request paths
pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
      create_table_order,
      add_item_to_table_order,
      get_all_table_order_items,
      get_remaining_table_order_items,
      cancel_item_from_table_order,
    ]
}