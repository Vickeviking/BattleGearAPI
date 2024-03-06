use crate::models::{NewTotalThrophies, TotalThrophies, User};
use crate::repositories::TotalThrophiesRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};

/*  TESTED  , 

*/

//------------- get endpoint -------------
//multi
#[rocket::get("/total_throphies")]
pub async fn get_total_throphies(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    TotalThrophiesRepository::find_multiple(&mut db, 100).await
        .map(|total_throphies| json!(total_throphies))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/total_throphies
*/

//single TotalThrophies
#[rocket::get("/total_throphies/<id>")]
pub async fn view_total_throphies(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    TotalThrophiesRepository::find(&mut db, id).await
        .map(|total_throphies| json!(total_throphies))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/total_throphies/1
*/

//------------- create endpoint -------------
#[rocket::post("/total_throphies", format="json", data="<new_total_throphies>")]
pub async fn create_total_throphies(mut db: Connection<DbConn>, new_total_throphies: Json<NewTotalThrophies>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    TotalThrophiesRepository::create(&mut db, new_total_throphies.into_inner()).await
        .map(|total_throphies| Custom(Status::Created, json!(total_throphies)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/total_throphies -H 'Content-type: application/json' 
  -d '{"user_id":1,"total_throphies":1000}'
*/

//------------- update endpoint -------------
#[rocket::put("/total_throphies/<id>", format="json", data="<total_throphies>")]
pub async fn update_total_throphies(mut db: Connection<DbConn>, id: i32, total_throphies: Json<TotalThrophies>, _user: User) -> Result<Value, Custom<Value>> {
    TotalThrophiesRepository::update(&mut db, id, total_throphies.into_inner()).await
        .map(|total_throphies| json!(total_throphies))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/total_throphies/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1,"total_throphies":1000}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/total_throphies/<id>")]
pub async fn delete_total_throphies(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    TotalThrophiesRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/total_throphies/1 -X DELETE 
*/