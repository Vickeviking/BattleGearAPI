use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};



/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The chats depend on users, while they store user_id, 
    1. so chats are deleted before users, to avoid foreign key constraint error
    2. chats are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/chats", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn create_test_chat(client: &Client, sender_id: i64, reciever_id: i64) -> Value {
    let response = client.post(format!("{}/chats", APP_HOST))
        .json(&json!({
            "sender_id":sender_id,
            "receiver_id":reciever_id,
            "message":"hello"
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let chats: Value = response.json().unwrap();
    println!("{:#?}", chats);
    chats
}

fn delete_test_chat(client: &Client, chat: Value) { 
    let response = client.delete(format!("{}/chats/{}", APP_HOST, chat["chat_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_chats() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    // friend with each other
    let chat1: Value = create_test_chat(&client, user1["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());
    let chat2: Value = create_test_chat(&client, user2["user_id"].as_i64().unwrap(), user1["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/chats", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&chat1));
    assert!(json.as_array().unwrap().contains(&chat2));

    // clean up
    delete_test_chat(&client, chat1);
    delete_test_chat(&client, chat2);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_chats() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com");
    let user2: Value = create_test_user(&client ,"testuser@gmail.com");
    let chat: Value = create_test_chat(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/chats/{}", APP_HOST, chat["chat_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let chat: Value = response.json().unwrap();
    assert_eq!(chat, json!({
        "chat_id": chat["chat_id"],
        "sender_id":user["user_id"].as_i64().unwrap(),
        "receiver_id":user2["user_id"].as_i64().unwrap(),
        "message":"hello",
        "timestamp": chat["timestamp"],
        "is_read": chat["is_read"]
    }));

    // clean up
    delete_test_chat(&client, chat);
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
}

#[test]
fn test_create_chats() {
    // test create chats
    let client = common::get_client_with_logged_in_admin();
    let user1 = create_test_user(&client, "testuser@gmail.com");
    let user2 = create_test_user(&client, "testuser@gmail.com");

    let response = client.post(format!("{}/chats", APP_HOST))
        .json(&json!({
            "sender_id":user1["user_id"].as_i64().unwrap(),
            "receiver_id":user2["user_id"].as_i64().unwrap(),
            "message":"hello"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let chat: Value = response.json().unwrap();
    assert_eq!(chat, json!({
        "chat_id": chat["chat_id"],
        "sender_id":user1["user_id"].as_i64().unwrap(),
        "receiver_id":user2["user_id"].as_i64().unwrap(),
        "message":"hello",
        "timestamp": chat["timestamp"],
        "is_read": chat["is_read"]
    }));

    // clean up
    delete_test_chat(&client, chat);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}


#[test]
fn test_update_chats() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let user3: Value = create_test_user(&client, "testuser@gmail.com");
    let chat: Value = create_test_chat(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());
    // test
    let response = client.put(format!("{}/chats/{}", APP_HOST, chat["chat_id"]))
        .json(&json!({
            "sender_id":user["user_id"].as_i64().unwrap(),
            "receiver_id":user3["user_id"].as_i64().unwrap(),
            "message":"hello"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let chat: Value = response.json().unwrap();
    assert_eq!(chat, json!({
        "chat_id": chat["chat_id"],
        "sender_id":user["user_id"].as_i64().unwrap(),
        "receiver_id":user3["user_id"].as_i64().unwrap(),
        "message":"hello",
        "timestamp": chat["timestamp"],
        "is_read": chat["is_read"]
    }));

    // clean up
    delete_test_chat(&client, chat);
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
    delete_test_user(&client, user3);

}

#[test]
fn test_delete_chats() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let chat: Value = create_test_chat(&client, user["user_id"].as_i64().unwrap(), user2["user_id"].as_i64().unwrap());

    let response = client.delete(format!("{}/chats/{}", APP_HOST, chat["chat_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
    delete_test_user(&client, user);
    delete_test_user(&client, user2);
}