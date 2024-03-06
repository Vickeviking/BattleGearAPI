use crate::auth::hash_password;
use crate::models;
use crate::repositories;
use diesel_async::{AsyncConnection, AsyncPgConnection};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url =
        std::env::var("DATABASE_URL").expect("Cannot find DATABASE_URL in environment variables");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Cannot connect to postgres")
}

pub async fn create_user(
    username: String,
    email: String,
    password: String,
    full_name: String,
    country: String,
    date_of_birth: String,
    role_codes: Vec<String>,
) {
    let mut c = load_db_connection().await;
    let password_hashed = hash_password(password).unwrap();

    let new_user = models::NewUser {
        username,
        email,
        password_hash: password_hashed,
        full_name: Some(full_name),
        country: Some(country),
        date_of_birth: Some(date_of_birth.parse().unwrap()),
    };
    let user = repositories::UserRepository::create(&mut c, new_user, role_codes)
        .await
        .unwrap();
    println!("Created user: {:?}", user);
    let roles = repositories::RoleRepository::find_by_user(&mut c, &user)
        .await
        .unwrap();
    println!("Roles Assigned: {:?}", roles);
}

pub async fn list_users() {
    let mut c = load_db_connection().await;

    let users = repositories::UserRepository::find_with_roles(&mut c)
        .await
        .unwrap();
    for user in users {
        println!("User: {:?}", user);
    }
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    let user = repositories::UserRepository::delete(&mut c, id)
        .await
        .unwrap();
    println!("Deleted user: {:?}", user);
}

pub async fn delete_user_by_username(username: String) {
    let mut c = load_db_connection().await;
    let user = repositories::UserRepository::delete_by_username(&mut c, &username)
        .await
        .unwrap();
    println!("Deleted user: {:?}", user);
}
