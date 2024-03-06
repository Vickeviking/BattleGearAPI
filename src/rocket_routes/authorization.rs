use crate::auth::{authorize_user, Credentials};
use crate::repositories::{UserRepository, SessionRepository};
use crate::rocket_routes::{server_error, DbConn, CacheConn};
use rocket::response::status::Custom;
use rocket::http::Status;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;



#[rocket::post("/login", format="json", data="<credentials>")]
pub async fn login(mut db: Connection<DbConn>, mut cache: Connection<CacheConn>, credentials: Json<Credentials>) -> Result<Value, Custom<Value>> {
    // query the database for the user
    let user = UserRepository::find_by_username(&mut db, &credentials.username).await
        .map_err(|e| server_error(e.into()))?;

    let session_id = authorize_user(&user, credentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Invalid credentials")))?;

    // create a session in the cache
    SessionRepository::create_session(&mut cache, session_id.clone(), user.user_id).await?;

    Ok(json!({
        "token": session_id,
    }))
}
/* Tested with , works
    docker-compose exec app curl 127.0.0.1:8000/login 
    -d '{"username":"admin2", "password":"admin"}' 
    -H 'Content-type: application/json'
*/