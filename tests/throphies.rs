use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};


/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The throphies depend on users, while they store user_id, 
    1. so throphies are deleted before users, to avoid foreign key constraint error
    2. throphies are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/throphies", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


fn create_test_throphies(client: &Client, user_id: i64) -> Value {
    let response = client.post(format!("{}/throphies", APP_HOST))
        .json(&json!({
            "user_id":user_id,
            "points":1000
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let throphies: Value = response.json().unwrap();
    println!("{:#?}", throphies);
    throphies
}

fn delete_test_throphies(client: &Client, throphies: Value) {
    let response = client.delete(format!("{}/throphies/{}", APP_HOST, throphies["trophy_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_throphies() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let throphies1: Value = create_test_throphies(&client, user1["user_id"].as_i64().unwrap());
    let throphies2: Value = create_test_throphies(&client, user2["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/throphies", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&throphies1));
    assert!(json.as_array().unwrap().contains(&throphies2));

    // clean up
    delete_test_throphies(&client, throphies1);
    delete_test_throphies(&client, throphies2);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_throphies() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com");
    let throphies: Value = create_test_throphies(&client, user["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/throphies/{}", APP_HOST, throphies["trophy_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let throphies: Value = response.json().unwrap();
    assert_eq!(throphies, json!({
        "trophy_id": throphies["trophy_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "points":1000,
        "game_timestamp":throphies["game_timestamp"]
    }));

    // clean up
    delete_test_throphies(&client, throphies);
    delete_test_user(&client, user);
}

#[test]
fn test_create_throphies() {
    // test create throphies
    let client = common::get_client_with_logged_in_admin();
    let user1 = create_test_user(&client, "testuser@gmail.com");

    let response = client.post(format!("{}/throphies", APP_HOST))
        .json(&json!({
            "user_id":user1["user_id"].as_i64().unwrap(),
            "points":1000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let throphies: Value = response.json().unwrap();
    assert_eq!(throphies, json!({
        "trophy_id": throphies["trophy_id"],
        "user_id":user1["user_id"].as_i64().unwrap(),
        "points":1000,
        "game_timestamp":throphies["game_timestamp"]
    }));

    // clean up
    delete_test_throphies(&client, throphies);
    delete_test_user(&client, user1);
}


#[test]
fn test_update_throphies() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let throphies: Value = create_test_throphies(&client, user["user_id"].as_i64().unwrap());
    // test
    let response = client.put(format!("{}/throphies/{}", APP_HOST, throphies["trophy_id"]))
        .json(&json!({
            "user_id":user["user_id"].as_i64().unwrap(),
            "points":2000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let throphies: Value = response.json().unwrap();
    assert_eq!(throphies, json!({
        "trophy_id": throphies["trophy_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "points":2000,
        "game_timestamp":throphies["game_timestamp"]
    }));

    // clean up
    delete_test_throphies(&client, throphies);
    delete_test_user(&client, user);

}

#[test]
fn test_delete_throphies() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let throphies: Value = create_test_throphies(&client, user["user_id"].as_i64().unwrap());

    let response = client.delete(format!("{}/throphies/{}", APP_HOST, throphies["trophy_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
    delete_test_user(&client, user);
}