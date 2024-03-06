use crate::models::{NewFriendship, Friendship, User};
use crate::repositories::FriendshipRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};


/*  TESTED  , 
*/

//------------- get endpoint -------------
//multi
#[rocket::get("/friendships")]
pub async fn get_friendships(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    FriendshipRepository::find_multiple(&mut db, 100).await
        .map(|friendships| json!(friendships))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/friendships
*/

//single friendship
#[rocket::get("/friendships/<id>")]
pub async fn view_friendship(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    FriendshipRepository::find(&mut db, id).await
        .map(|friendship| json!(friendship))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/friendships/1
*/

//------------- create endpoint -------------
#[rocket::post("/friendships", format="json", data="<new_friendship>")]
pub async fn create_friendship(mut db: Connection<DbConn>, new_friendship: Json<NewFriendship>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    FriendshipRepository::create(&mut db, new_friendship.into_inner()).await
        .map(|friendship| Custom(Status::Created, json!(friendship)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/friendships -H 'Content-type: application/json' 
  -d '{"user_id":1,"friend_id":2,"status":"accepted"}'
*/

//------------- update endpoint -------------
#[rocket::put("/friendships/<id>", format="json", data="<friendship>")]
pub async fn update_friendship(mut db: Connection<DbConn>, id: i32, friendship: Json<Friendship>, _user: User) -> Result<Value, Custom<Value>> {
    FriendshipRepository::update(&mut db, id, friendship.into_inner()).await
        .map(|friendship| json!(friendship))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/friendships/1 -X PUT -H 'Content-type: application/json' 
  -d '{"user_id":1, "friend_id":2, "status":"blocked"}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/friendships/<id>")]
pub async fn delete_friendship(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    FriendshipRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/friendships/1 -X DELETE 
*/