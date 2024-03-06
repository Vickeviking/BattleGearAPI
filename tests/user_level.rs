use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};


/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The user_levels depend on users, while they store user_id, 
    1. so user_levels are deleted before users, to avoid foreign key constraint error
    2. user_levels are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/user_levels", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn create_test_user_level(client: &Client, user_id: i64) -> Value {
    let response = client.post(format!("{}/user_levels", APP_HOST))
        .json(&json!({
            "user_id":user_id,
            "level":1,
            "experience_points":1000
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let user_level: Value = response.json().unwrap();
    println!("{:#?}", user_level);
    user_level
}

fn delete_test_user_level(client: &Client, user_level: Value) {
    let response = client.delete(format!("{}/user_levels/{}", APP_HOST, user_level["user_level_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_user_levels() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let user_level1: Value = create_test_user_level(&client, user1["user_id"].as_i64().unwrap());
    let user_level2: Value = create_test_user_level(&client, user2["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/user_levels", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&user_level1));
    assert!(json.as_array().unwrap().contains(&user_level2));

    // clean up
    delete_test_user_level(&client, user_level1);
    delete_test_user_level(&client, user_level2);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_user_levels() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com");
    let user_level: Value = create_test_user_level(&client, user["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/user_levels/{}", APP_HOST, user_level["user_level_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let user_level: Value = response.json().unwrap();
    assert_eq!(user_level, json!({
        "user_level_id": user_level["user_level_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "level":1,
        "experience_points":1000
    }));

    // clean up
    delete_test_user_level(&client, user_level);
    delete_test_user(&client, user);
}

#[test]
fn test_create_user_levels() {
    // test create user_levels
    let client = common::get_client_with_logged_in_admin();
    let user1 = create_test_user(&client, "testuser@gmail.com");

    let response = client.post(format!("{}/user_levels", APP_HOST))
        .json(&json!({
            "user_id":user1["user_id"].as_i64().unwrap(),
            "level":1,
            "experience_points":1000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let user_level: Value = response.json().unwrap();
    assert_eq!(user_level, json!({
        "user_level_id": user_level["user_level_id"],
        "user_id":user1["user_id"].as_i64().unwrap(),
        "level":1,
        "experience_points":1000
    }));

    // clean up
    delete_test_user_level(&client, user_level);
    delete_test_user(&client, user1);
}


#[test]
fn test_update_user_levels() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user_level: Value = create_test_user_level(&client, user["user_id"].as_i64().unwrap());
    // test
    let response = client.put(format!("{}/user_levels/{}", APP_HOST, user_level["user_level_id"]))
        .json(&json!({
            "user_id":user["user_id"].as_i64().unwrap(),
            "level":1,
            "experience_points":2000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let user_level: Value = response.json().unwrap();
    assert_eq!(user_level, json!({
        "user_level_id": user_level["user_level_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "level":1,
        "experience_points":2000
    }));

    // clean up
    delete_test_user_level(&client, user_level);
    delete_test_user(&client, user);

}

#[test]
fn test_delete_user_levels() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user_level: Value = create_test_user_level(&client, user["user_id"].as_i64().unwrap());

    let response = client.delete(format!("{}/user_levels/{}", APP_HOST, user_level["user_level_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
    delete_test_user(&client, user);
}







