use crate::models::{NewCurrency, Currency, User};
use crate::repositories::CurrencyRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};

/*  TESTED  , 


*/

//------------- get endpoint -------------
//multi
#[rocket::get("/currencies")]
pub async fn get_currencies(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    CurrencyRepository::find_multiple(&mut db, 100).await
        .map(|currencies| json!(currencies))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/currencies
*/

//single currency
#[rocket::get("/currencies/<id>")]
pub async fn view_currency(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    CurrencyRepository::find(&mut db, id).await
        .map(|currency| json!(currency))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/currencies/1
*/

//------------- create endpoint -------------
#[rocket::post("/currencies", format="json", data="<new_currency>")]
pub async fn create_currency(mut db: Connection<DbConn>, new_currency: Json<NewCurrency>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    CurrencyRepository::create(&mut db, new_currency.into_inner()).await
        .map(|currency| Custom(Status::Created, json!(currency)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/currencies -H 'Content-type: application/json' 
  -d '{"user_id":1,"currency_type":"gold","amount":1000}'
*/

//------------- update endpoint -------------
#[rocket::put("/currencies/<id>", format="json", data="<currency>")]
pub async fn update_currency(mut db: Connection<DbConn>, id: i32, currency: Json<Currency>, _user: User) -> Result<Value, Custom<Value>> {
    CurrencyRepository::update(&mut db, id, currency.into_inner()).await
        .map(|currency| json!(currency))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/currencies/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1,"currency_type":"gold","amount":1000}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/currencies/<id>")]
pub async fn delete_currency(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    CurrencyRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/currencies/1 -X DELETE 
*/