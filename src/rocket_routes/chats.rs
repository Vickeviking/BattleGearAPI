use crate::models::{Chat, NewChat, User};
use crate::repositories::ChatRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};



/*  TESTED  , 


*/

//------------- get endpoint -------------
//multi
#[rocket::get("/chats")]
pub async fn get_chats(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    ChatRepository::find_multiple(&mut db, 100).await
        .map(|chats| json!(chats))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/chats
*/

//single chat
#[rocket::get("/chats/<id>")]
pub async fn view_chat(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    ChatRepository::find(&mut db, id).await
        .map(|chat| json!(chat))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/chats/1

*/

//------------- create endpoint -------------
#[rocket::post("/chats", format="json", data="<new_chat>")]
pub async fn create_chat(mut db: Connection<DbConn>, new_chat: Json<NewChat>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    ChatRepository::create(&mut db, new_chat.into_inner()).await
        .map(|chat| Custom(Status::Created, json!(chat)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/chats -H 'Content-type: application/json' 
  -d '{"sender_id":1,"receiver_id":2,"message":"hello"}'
*/

//------------- update endpoint -------------
#[rocket::put("/chats/<id>", format="json", data="<chat>")]
pub async fn update_chat(mut db: Connection<DbConn>, id: i32, chat: Json<Chat>, _user: User) -> Result<Value, Custom<Value>> {
    ChatRepository::update(&mut db, id, chat.into_inner()).await
        .map(|chat| json!(chat))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/chats/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1, "friend_id":2, "status":"accepted", "friendship_date":"2021-01-01"}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/chats/<id>")]
pub async fn delete_chat(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    ChatRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/chats/1 -X DELETE 
*/