use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};

/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The friendships depend on users, while they store user_id, 
    1. so friendships are deleted before users, to avoid foreign key constraint error
    2. friendships are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/friendships", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn create_test_friendships(client: &Client, user_id: i64, friend_id: i64) -> Value {
    let response = client.post(format!("{}/friendships", APP_HOST))
        .json(&json!({
            "user_id":user_id,
            "friend_id":friend_id,
            "status":"accepted",
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let friendships: Value = response.json().unwrap();
    println!("{:#?}", friendships);
    friendships
}

fn delete_test_friendship(client: &Client, friendship: Value) {
    let response = client.delete(format!("{}/friendships/{}", APP_HOST, friendship["friendship_id"])
)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_friendships() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    // friend with each other
    let friendship1: Value = create_test_friendships(&client, user1["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());
    let friendship2: Value = create_test_friendships(&client, user2["user_id"].as_i64().unwrap(), user1["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/friendships", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&friendship1));
    assert!(json.as_array().unwrap().contains(&friendship2));

    // clean up
    delete_test_friendship(&client, friendship1);
    delete_test_friendship(&client, friendship2);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_friendships() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com");
    let user2: Value = create_test_user(&client ,"testuser@gmail.com");
    let friendship: Value = create_test_friendships(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/friendships/{}", APP_HOST, friendship["friendship_id"])
)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let friendship: Value = response.json().unwrap();
    assert_eq!(friendship, json!({
        "friendship_id": friendship["friendship_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "friend_id":user2["user_id"].as_i64().unwrap(),
        "status":"accepted",
        "friendship_date": friendship["friendship_date"]
    }));

    // clean up
    delete_test_friendship(&client, friendship);
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
}

#[test]
fn test_create_friendships() {
    // test create friendships
    let client = common::get_client_with_logged_in_admin();
    let user1 = create_test_user(&client, "testuser@gmail.com");
    let user2 = create_test_user(&client, "testuser@gmail.com");

    let response = client.post(format!("{}/friendships", APP_HOST))
        .json(&json!({
            "user_id":user1["user_id"].as_i64().unwrap(),
            "friend_id":user2["user_id"].as_i64().unwrap(),
            "status":"accepted"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let friendship: Value = response.json().unwrap();
    assert_eq!(friendship, json!({
        "friendship_id": friendship["friendship_id"],
        "user_id":user1["user_id"].as_i64().unwrap(),
        "friend_id":user2["user_id"].as_i64().unwrap(),
        "status":"accepted",
        "friendship_date": friendship["friendship_date"]
    }));

    // clean up
    delete_test_friendship(&client, friendship);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}


#[test]
fn test_update_friendships() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let user3: Value = create_test_user(&client, "testuser@gmail.com");
    let friendship: Value = create_test_friendships(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());
    // test
    let response = client.put(format!("{}/friendships/{}", APP_HOST, friendship["friendship_id"])
)
        .json(&json!({
            "user_id":user["user_id"].as_i64().unwrap(),
            "friend_id":user3["user_id"].as_i64().unwrap(),
            "status":"accepted"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let friendship: Value = response.json().unwrap();
    assert_eq!(friendship, json!({
        "friendship_id": friendship["friendship_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "friend_id":user3["user_id"].as_i64().unwrap(),
        "status":"accepted",
        "friendship_date": friendship["friendship_date"]
    }));

    // clean up
    delete_test_friendship(&client, friendship);
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
    delete_test_user(&client, user3);

}

#[test]
fn test_delete_friendships() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let friendship: Value = create_test_friendships(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());

    let response = client.delete(format!("{}/friendships/{}", APP_HOST, friendship["friendship_id"])
)
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
}