extern crate battle_gear as api_server;

use rocket_db_pools::Database;
use std::env;

#[rocket::main]
async fn main() {
    // Set the ROCKET_ADDRESS environment variable to 0.0.0.0
    env::set_var("ROCKET_ADDRESS", "0.0.0.0");
    let _ = rocket::build()
        .mount("/", rocket::routes![
            //authorization
            api_server::rocket_routes::authorization::login,
            //chats
            api_server::rocket_routes::chats::get_chats,
            api_server::rocket_routes::chats::view_chat,
            api_server::rocket_routes::chats::create_chat,
            api_server::rocket_routes::chats::update_chat,
            api_server::rocket_routes::chats::delete_chat,
            //currencies
            api_server::rocket_routes::currency::get_currencies,
            api_server::rocket_routes::currency::view_currency,
            api_server::rocket_routes::currency::create_currency,
            api_server::rocket_routes::currency::update_currency,
            api_server::rocket_routes::currency::delete_currency,
            //friendships
            api_server::rocket_routes::friendships::get_friendships,
            api_server::rocket_routes::friendships::view_friendship,
            api_server::rocket_routes::friendships::create_friendship,
            api_server::rocket_routes::friendships::update_friendship,
            api_server::rocket_routes::friendships::delete_friendship,
            //images
            api_server::rocket_routes::images::get_images,
            api_server::rocket_routes::images::view_image,
            api_server::rocket_routes::images::create_image,
            api_server::rocket_routes::images::update_image,
            api_server::rocket_routes::images::delete_image,
            //throphies
            api_server::rocket_routes::throphies::get_throphies,
            api_server::rocket_routes::throphies::view_throphy,
            api_server::rocket_routes::throphies::create_throphy,
            api_server::rocket_routes::throphies::update_throphy,
            api_server::rocket_routes::throphies::delete_throphy,
            //total_throphies
            api_server::rocket_routes::total_throphies::get_total_throphies,
            api_server::rocket_routes::total_throphies::view_total_throphies,
            api_server::rocket_routes::total_throphies::create_total_throphies,
            api_server::rocket_routes::total_throphies::update_total_throphies,
            api_server::rocket_routes::total_throphies::delete_total_throphies,
            //user_level
            api_server::rocket_routes::user_level::get_user_levels,
            api_server::rocket_routes::user_level::view_user_levels,
            api_server::rocket_routes::user_level::create_user_levels,
            api_server::rocket_routes::user_level::update_user_levels,
            api_server::rocket_routes::user_level::delete_user_levels,
            //users
            api_server::rocket_routes::users::get_users,    
            api_server::rocket_routes::users::view_user,
            api_server::rocket_routes::users::create_user,
            api_server::rocket_routes::users::update_user,
            api_server::rocket_routes::users::delete_user,    
            api_server::rocket_routes::users::username_exists,
            api_server::rocket_routes::users::email_exists,     
        ])
        .attach(api_server::rocket_routes::CacheConn::init())
        .attach(api_server::rocket_routes::DbConn::init())
        .launch()
        .await;
}

