// For enabling DB access
use rocket::State;
// For enabling Rust struct <-> JSON ser-de
use rocket_contrib::json::Json;

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

// order routes

/// Place an order for a table
/// There would be only one order per table at a time
#[rocket::post("/table/create_order", data = "<new_table_order>")]
fn create_table_order(db: State<Db>, new_table_order: Json<NewTableOrder>) -> Result<Json<TableOrders>, StdErr> {
    // TODO: check and set table status before creating an order -> DONE
    let table = new_table_order.0;
    let result = db.is_table_free(table.table_id);

    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    if response[0].is_free { // if is_free == true, which is by default
        let result = db.set_table_occupied(table.table_id, false);
        match result {
            Ok(res) => res,
            Err(err) => return Err(err),
        };
        db.create_table_order(table).map(Json)
    } else {
        Err("Table already occupied.".into()) // Into trait for &str -> Box dyn err
    }
}

/// Add an item to a table
#[rocket::post("/table/add_item", data = "<new_order_item>")]
fn add_item_to_table_order(db: State<Db>, new_order_item: Json<NewTableOrderItem>) -> Result<Json<TableOrderItems>, StdErr> {
    // some error related to responder for enum type. Ref: https://github.com/serde-rs/serde/issues/912 -> solved by changing enum to fk table
    db.add_item_to_table_order(new_order_item.0).map(Json)
}

// #[rocket::post("/table/add_item", data = "<new_order_items>")]
// fn add_multi_items_to_table_order(db: State<Db>, new_order_items: Json<Vec<NewTableOrderItem>>) -> Result<Json<TableOrderItems>, StdErr> {
//     let result = Vec<TableOrderItems>;
//     for obj in new_order_items.iter() {
//         db.add_item_to_table_order(obj).map(Json);
//     }
// }


/// Get table's single order item details
/// Assumption: For same item ordered multiple times, will result in multiple entry response
#[rocket::get("/table/<table_id>/get_item/<item_id>")]
fn get_one_table_order_items(db: State<Db>, table_id: i64, item_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    db.get_one_table_order_item(response[0].order_id, item_id).map(Json)
}

/// Get table's all order item details
#[rocket::get("/table/<table_id>/all_items")]
fn get_all_table_order_items(db: State<Db>, table_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    db.get_all_table_order_items(response[0].order_id).map(Json)
}

/// Get remaining orders of a table
#[rocket::get("/table/<table_id>/remaining_items")]
fn get_remaining_table_order_items(db: State<Db>, table_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let result = db.get_order_id_from_table_id(table_id);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    db.get_remaining_table_order_items(response[0].order_id).map(Json)
}

/// Cancel item(s) from order of a table
/// Can be only canceled if the item is not already served or canceled
/// Because of the assumption that multiple instances of same item can be ordered,
/// this function cancels all instances of ordered item
#[rocket::post("/table/<table_id>/cancel_item/<item_id>")]
fn cancel_item_from_table_order(db: State<Db>, table_id: i64, item_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let mut return_vec: Vec<TableOrderItems> = Vec::new();
    let result = db.get_order_id_from_table_id(table_id);
    let order_response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let item_status = db.get_table_order_item_status_id(order_response[0].order_id, item_id);
    let status_set_response = match item_status{
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    for status_set in status_set_response.iter() {
        if status_set.item_status_id == 1 { // 1 for preparing
            let result = db.cancel_item_from_table_order(status_set.id);
            let curr_response = match result {
                Ok(res) => res,
                Err(err) => return Err(err),
            };
            return_vec.push(curr_response);
        } else {
            println!("Item cannot be canceled.");
        }
    }
     Ok(Json(return_vec))
}

/// Serve(remove) item(s) from order of a table
/// Can be only served if the item is in ""preparing" state
/// Because of the assumption that multiple instances of same item can be ordered,
/// this function serves all instances of ordered item
#[rocket::post("/table/<table_id>/serve_item/<item_id>")]
fn serve_item_from_table_order(db: State<Db>, table_id: i64, item_id: i64) -> Result<Json<Vec<TableOrderItems>>, StdErr> {
    let mut return_vec: Vec<TableOrderItems> = Vec::new();
    let result = db.get_order_id_from_table_id(table_id);
    let order_response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    let item_status = db.get_table_order_item_status_id(order_response[0].order_id, item_id);
    let status_set_response = match item_status{
        Ok(res) => res,
        Err(err) => return Err(err),
    };

    for status_set in status_set_response.iter() {
        if status_set.item_status_id == 1 { // 1 for preparing
            let result = db.serve_item_from_table_order(status_set.id);
            let curr_response = match result {
                Ok(res) => res,
                Err(err) => return Err(err),
            };
            return_vec.push(curr_response);
        } else {
            println!("Item cannot be served as it is either already served or canceled.");
        }
    }
     Ok(Json(return_vec))
}

// Pub function to return API routes; 'api/' needs to be used in request paths
pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
      create_table_order,
      add_item_to_table_order,
      get_one_table_order_items,
      get_all_table_order_items,
      get_remaining_table_order_items,
      cancel_item_from_table_order,
      serve_item_from_table_order,
    ]
}