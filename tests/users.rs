use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};


/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/users", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn test_get_users() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");

    // test
    let response = client.get(format!("{}/users", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&user1));
    assert!(json.as_array().unwrap().contains(&user2));

    // clean up
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_user() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com") ;

    // test
    let response = client.get(format!("{}/users/{}", APP_HOST, user["user_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let user: Value = response.json().unwrap();
    assert_eq!(user, json!({
        "user_id": user["user_id"],
        "username": user["username"],
        "email":user["email"],
        "password_hash":"testpassword",
        "full_name":"Test User",
        "avatar_id": user["avatar_id"],
        "registration_date": user["registration_date"],
        "last_login": user["last_login"],
        "is_active": user["is_active"],
        "is_admin": user["is_admin"],
        "timezone": user["timezone"],
        "language": user["language"],
        "country":"USA",
        "date_of_birth":"1990-01-01",
        "two_factor_auth_enabled": user["two_factor_auth_enabled"],
        "last_password_change": user["last_password_change"]
    }));

    // clean up
    delete_test_user(&client, user);

}

#[test]
fn test_create_users() {
    // test create user
    let client = common::get_client_with_logged_in_admin();
    let response = client.post(format!("{}/users", APP_HOST))
        .json(&json!({
            "username":"testuser123",
            "email":"testuser444@gmail.com",
            "password_hash":"testpassword",
            "full_name":"Test User",
            "country":"USA",
            "date_of_birth":"1990-01-01"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let user: Value = response.json().unwrap();
    assert_eq!(user, json!({
        "user_id": user["user_id"],
        "username":"testuser123",
        "email":user["email"],
        "password_hash":"testpassword",
        "full_name":"Test User",
        "avatar_id": user["avatar_id"],
        "registration_date": user["registration_date"],
        "last_login": user["last_login"],
        "is_active": user["is_active"],
        "is_admin": user["is_admin"],
        "timezone": user["timezone"],
        "language": user["language"],
        "country":"USA",
        "date_of_birth":"1990-01-01",
        "two_factor_auth_enabled": user["two_factor_auth_enabled"],
        "last_password_change": user["last_password_change"]
    }));

    // clean up
    delete_test_user(&client, user);
}


#[test]
fn test_update_user() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    // test
    let response = client.put(format!("{}/users/{}", APP_HOST, user["user_id"]))
        .json(&json!({
            "username":"testuser222",
            "email":"testuser555@gmail.com",
            "password_hash":"test2password",
            "full_name":"Test2 User2",
            "country":"USA2",
            "date_of_birth":"1990-01-01"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let user: Value = response.json().unwrap();
    assert_eq!(user, json!({
        "user_id": user["user_id"],
        "username":"testuser222",
        "email":user["email"],
        "password_hash":"test2password",
        "full_name":"Test2 User2",
        "avatar_id": user["avatar_id"],
        "registration_date": user["registration_date"],
        "last_login": user["last_login"],
        "is_active": user["is_active"],
        "is_admin": user["is_admin"],
        "timezone": user["timezone"],
        "language": user["language"],
        "country":"USA2",
        "date_of_birth":"1990-01-01",
        "two_factor_auth_enabled": user["two_factor_auth_enabled"],
        "last_password_change": user["last_password_change"]
    }));

    // clean up
    delete_test_user(&client, user);

}

#[test]
fn test_delete_user() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");

    let response = client.delete(format!("{}/users/{}", APP_HOST, user["user_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
}







