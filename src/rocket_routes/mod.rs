use std::error::Error;
use rocket::http::Status;
use rocket::serde::json::{json, Value};
use rocket::response::status::Custom;
use rocket::request::{FromRequest, Outcome};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;


use crate::models::User;
use crate::repositories::UserRepository;

pub mod authorization;
pub mod chats;
pub mod currency;
pub mod friendships;
pub mod images;
pub mod throphies;
pub mod total_throphies;
pub mod user_level;
pub mod users;

#[derive(rocket_db_pools::Database)]
#[database("postgres")]
pub struct DbConn(rocket_db_pools::diesel::PgPool);

#[derive(rocket_db_pools::Database)]
#[database("redis")]
pub struct CacheConn(rocket_db_pools::deadpool_redis::Pool);

pub fn server_error(e: Box<dyn Error>) -> Custom<Value> {
    rocket::error!("{}", e);
    Custom(Status::InternalServerError, json!("Error"))
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        // Authorization: Bearer SESSION_ID_128_CHARACTERS_LONG
        let session_header = request.headers().get_one("Authorization")
            .map(|v| v.split_whitespace().collect::<Vec<_>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");
        if let Some(header_value) = session_header {
            // query redis with header_value
            //lets retrieve handle to redis pool
            let mut cache = request.guard::<Connection<CacheConn>>().await
                .expect("Cache connection guard failed");
            let mut db = request.guard::<Connection<DbConn>>().await
                .expect("Db connection guard failed");

            let result = cache.get::<String, i32>(format!("sessions/{}", header_value[1])).await;
            if let Ok(user_id) = result {
                if let Ok(user) = UserRepository::find(&mut db, user_id).await {
                    return Outcome::Success(user);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}