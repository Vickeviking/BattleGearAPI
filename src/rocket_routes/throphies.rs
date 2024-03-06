use crate::models::{NewTrophy, Trophy, User};
use crate::repositories::ThrophiesRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};


/*  TESTED  , 


*/

//------------- get endpoint -------------
//multi
#[rocket::get("/throphies")]
pub async fn get_throphies(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    ThrophiesRepository::find_multiple(&mut db, 100).await
        .map(|throphies: Vec<Trophy>| json!(throphies))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/throphies
*/

//single throphy
#[rocket::get("/throphies/<id>")]
pub async fn view_throphy(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    ThrophiesRepository::find(&mut db, id).await
        .map(|throphy| json!(throphy))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/throphies/1
*/

//------------- create endpoint -------------
#[rocket::post("/throphies", format="json", data="<new_throphy>")]
pub async fn create_throphy(mut db: Connection<DbConn>, new_throphy: Json<NewTrophy>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    ThrophiesRepository::create(&mut db, new_throphy.into_inner()).await
        .map(|throphy| Custom(Status::Created, json!(throphy)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/throphies -H 'Content-type: application/json' 
  -d '{"user_id":1,"points":1000}'
*/

//------------- update endpoint -------------
#[rocket::put("/throphies/<id>", format="json", data="<throphy>")]
pub async fn update_throphy(mut db: Connection<DbConn>, id: i32, throphy: Json<Trophy>, _user: User) -> Result<Value, Custom<Value>> {
    ThrophiesRepository::update(&mut db, id, throphy.into_inner()).await
        .map(|throphy| json!(throphy))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/throphies/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1,"points":1000}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/throphies/<id>")]
pub async fn delete_throphy(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    ThrophiesRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/throphies/1 -X DELETE 
*/

