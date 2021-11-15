use rocket::http;
use rocket::response;
use rocket::request;

// For enabling DB access
use rocket::State;
// For enabling Rust struct <-> JSON ser-de
use rocket_contrib::json::Json;

use crate::StdErr;
use crate::db::Db;
use crate::models::*;

// order routes

#[rocket::post("/orders", data = "<new_table_order>")]
fn create_table_order(db: State<Db>, new_table_order: Json<NewTableOrder>) -> Result<Json<TableOrders>, StdErr> {
  println!("{:?}", new_table_order);
    db.create_table_order(new_table_order.0).map(Json)
}

// Pub function to return API routes; 'api/' needs to be used in request paths
pub fn api() -> Vec<rocket::Route> {
    rocket::routes![
      create_table_order,
    ]
}