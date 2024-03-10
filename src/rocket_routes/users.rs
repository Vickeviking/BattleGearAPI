use crate::models::{NewUser, User};
use crate::repositories::UserRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::response::status::NoContent;
use rocket::{response::status::Custom, serde::json::Json};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};



/*  TESTED 2024-02-17 , ALL ENDPOINTS WORKING✅

    Before testing the endpoints, make sure to have the following:
    1. Run the app with: docker-compose up
    2. Create the database with: diesel migration run
    3. control the migrations are running: diesel migration list

    watch logs and stuff with:
    docker-compose exec app cargo run

*/


//------------- get endpoint -------------
//multi
#[rocket::get("/users")]
pub async fn get_users(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    UserRepository::find_multiple(&mut db, 100).await
        .map(|users| json!(users))
        .map_err(|e| server_error(e.into()))
}   
/*
    Test Endpoint with:  Working✅
    docker-compose exec app curl 127.0.0.1:8000/users
*/

//single user
#[rocket::get("/users/<id>")]
pub async fn view_user(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    UserRepository::find(&mut db, id).await
        .map(|user| json!(user))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with:  Working✅
    docker-compose exec app curl 127.0.0.1:8000/users/1
*/

//------------- create endpoint -------------
#[rocket::post("/users", format="json", data="<new_user>")]
pub async fn create_user(mut db: Connection<DbConn>, new_user: Json<NewUser>) -> Result<Custom<Value>, Custom<Value>> {
    UserRepository::create(&mut db, new_user.into_inner(), vec![]).await
        .map(|user| Custom(Status::Created, json!(user)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  Working✅
  docker-compose exec app curl 127.0.0.1:8000/users -X POST -H 'Content-type: application/json' 
  -d '{"username":"testuser","email":"testuser@gmail.com",
       "password_hash":"testpassword","full_name":"Test User",
       "country":"USA","date_of_birth":"1990-01-01"}'
*/

//------------- update endpoint -------------
#[rocket::put("/users/<id>", format="json", data="<user>")]
pub async fn update_user(mut db: Connection<DbConn>, id: i32, user: Json<User>, _user: User) -> Result<Value, Custom<Value>> {
    UserRepository::update(&mut db, id, user.into_inner()).await
        .map(|user| json!(user))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  working✅
  docker-compose exec app curl 127.0.0.1:8000/users/1 -X PUT -H 'Content-type: application/json' 
  -d '{"auth_token":null,"avatar_id":null,"country":"USA","date_of_birth":"1990-01-01","email":"testuser@gmail.com","full_name":"Test User","is_active":true,"is_admin":false,"language":null,"last_login":null,"last_password_change":null,"password_hash":"testpassword","registration_date":"2024-02-16T23:47:54.691633","timezone":null,"two_factor_auth_enabled":false,"user_id":1,"username":"testuserupdated"}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/users/<id>")]
pub async fn delete_user(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    UserRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  working✅
  docker-compose exec app curl 127.0.0.1:8000/users/1 -X DELETE 
*/

// ------------- username exists endpoint -------------
#[rocket::get("/users/username_exists/<username>")]
pub async fn username_exists(mut db: Connection<DbConn>, username: String) -> Result<Json<bool>, Custom<Value>> {
    UserRepository::username_exists(&mut db, username).await
        .map(Json)
        .map_err(|e| server_error(e.into()))
}

// ------------- email exists endpoint -------------
#[rocket::get("/users/email_exists/<email>")]
pub async fn email_exists(mut db: Connection<DbConn>, email: String) -> Result<Json<bool>, Custom<Value>> {
    UserRepository::email_exists(&mut db, email).await
        .map(Json)
        .map_err(|e| server_error(e.into()))
}