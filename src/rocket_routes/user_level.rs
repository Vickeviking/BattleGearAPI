use crate::models::{NewUserLevel, UserLevel, User};
use crate::repositories::UserLevelRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};



/*  TESTED  , 

*/

//------------- get endpoint -------------
//multi
#[rocket::get("/user_levels")]
pub async fn get_user_levels(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    UserLevelRepository::find_multiple(&mut db, 100).await
        .map(|user_levels| json!(user_levels))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/user_levels
*/

//single user_level
#[rocket::get("/user_levels/<id>")]
pub async fn view_user_levels(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    UserLevelRepository::find(&mut db, id).await
        .map(|user_level| json!(user_level))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/user_levels/1
*/

//------------- create endpoint -------------
#[rocket::post("/user_levels", format="json", data="<new_user_level>")]
pub async fn create_user_levels(mut db: Connection<DbConn>, new_user_level: Json<NewUserLevel>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    UserLevelRepository::create(&mut db, new_user_level.into_inner()).await
        .map(|user_level| Custom(Status::Created, json!(user_level)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/user_levels -H 'Content-type: application/json' 
  -d '{"user_id":1,"level":1,"experience_points":1000}'
*/

//------------- update endpoint -------------
#[rocket::put("/user_levels/<id>", format="json", data="<user_level>")]
pub async fn update_user_levels(mut db: Connection<DbConn>, id: i32, user_level: Json<UserLevel>, _user: User) -> Result<Value, Custom<Value>> {
    UserLevelRepository::update(&mut db, id, user_level.into_inner()).await
        .map(|user_level| json!(user_level))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/user_levels/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1,"level":1,"experience_points":1000}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/user_levels/<id>")]
pub async fn delete_user_levels(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    UserLevelRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/user_levels/1 -X DELETE 
*/