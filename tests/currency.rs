use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::{create_test_user, delete_test_user, APP_HOST};

/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The currencies depend on users, while they store user_id, 
    1. so currencies are deleted before users, to avoid foreign key constraint error
    2. currencies are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/currencies", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

fn create_test_currency(client: &Client, user_id: i64) -> Value {
    let response = client.post(format!("{}/currencies", APP_HOST))
        .json(&json!({
            "user_id":user_id,
            "currency_type":"gold",
            "amount":1000
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let currencies: Value = response.json().unwrap();
    println!("{:#?}", currencies);
    currencies
}

fn delete_test_currency(client: &Client, currency: Value) {
    let response = client.delete(format!("{}/currencies/{}", APP_HOST, currency["currency_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_currencies() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let user1: Value = create_test_user(&client, "testuser@gmail.com");
    let user2: Value = create_test_user(&client, "testuser@gmail.com");
    let currency1: Value = create_test_currency(&client, user1["user_id"].as_i64().unwrap());
    let currency2: Value = create_test_currency(&client, user2["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/currencies", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&currency1));
    assert!(json.as_array().unwrap().contains(&currency2));

    // clean up
    delete_test_currency(&client, currency1);
    delete_test_currency(&client, currency2);
    delete_test_user(&client, user1);
    delete_test_user(&client, user2);
}

#[test]
fn test_view_currencies() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client ,"testuser@gmail.com");
    let currency: Value = create_test_currency(&client, user["user_id"].as_i64().unwrap());

    // test
    let response = client.get(format!("{}/currencies/{}", APP_HOST, currency["currency_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let currency: Value = response.json().unwrap();
    assert_eq!(currency, json!({
        "currency_id": currency["currency_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "currency_type":"gold",
        "amount":1000,
        "last_updated": currency["last_updated"]
    }));

    // clean up
    delete_test_currency(&client, currency);
    delete_test_user(&client, user);
}

#[test]
fn test_create_currencies() {
    // test create currencies
    let client = common::get_client_with_logged_in_admin();
    let user1 = create_test_user(&client, "testuser@gmail.com");

    let response = client.post(format!("{}/currencies", APP_HOST))
        .json(&json!({
            "user_id":user1["user_id"].as_i64().unwrap(),
            "currency_type":"gold",
            "amount":1000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let currency: Value = response.json().unwrap();
    assert_eq!(currency, json!({
        "currency_id": currency["currency_id"],
        "user_id":user1["user_id"].as_i64().unwrap(),
        "currency_type":"gold",
        "amount":1000,
        "last_updated": currency["last_updated"]
    }));

    // clean up
    delete_test_currency(&client, currency);
    delete_test_user(&client, user1);
}


#[test]
fn test_update_currencies() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let currency: Value = create_test_currency(&client, user["user_id"].as_i64().unwrap());
    // test
    let response = client.put(format!("{}/currencies/{}", APP_HOST, currency["currency_id"]))
        .json(&json!({
            "user_id":user["user_id"].as_i64().unwrap(),
            "currency_type":"gold",
            "amount":2000
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let currency: Value = response.json().unwrap();
    assert_eq!(currency, json!({
        "currency_id": currency["currency_id"],
        "user_id":user["user_id"].as_i64().unwrap(),
        "currency_type":"gold",
        "amount":2000,
        "last_updated": currency["last_updated"]
    }));

    // clean up
    delete_test_currency(&client, currency);
    delete_test_user(&client, user);

}

#[test]
fn test_delete_currencies() {
    let client = common::get_client_with_logged_in_admin();
    let user: Value = create_test_user(&client, "testuser@gmail.com");
    let currency: Value = create_test_currency(&client, user["user_id"].as_i64().unwrap());

    let response = client.delete(format!("{}/currencies/{}", APP_HOST, currency["currency_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
    delete_test_user(&client, user);
}