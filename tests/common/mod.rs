use serde_json::{json, Value};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};
use reqwest::blocking::{Client, ClientBuilder};
use reqwest::{header, StatusCode};



pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

static TEST_ID: AtomicUsize = AtomicUsize::new(0);

pub fn create_test_user(client: &Client, email: &str) -> Value {
    let test_id = TEST_ID.fetch_add(1, Ordering::SeqCst);
    let email_parts: Vec<&str> = email.split('@').collect();
    let unique_email = format!("{}{}@{}", email_parts[0], test_id, email_parts[1]);
    let unique_username = format!("testuser{}", test_id);
    let response = client.post("http://127.0.0.1:8000/users")
        .json(&json!({
            "username": unique_username,
            "email": unique_email,
            "password_hash":"testpassword",
            "full_name":"Test User",
            "country":"USA",
            "date_of_birth":"1990-01-01"
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let user: Value = response.json().unwrap();
    println!("{:#?}", user);
    user
}

pub fn delete_test_user(client: &Client, user: Value) {
    let response = client.delete(format!("http://127.0.0.1:8000/users/{}", user["user_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn get_client_with_logged_in_admin() -> Client {
    // create a user in db, then login add headers and return the client
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("testAdminUser")
        .arg("testAdminUser@gmail.com")
        .arg("testAdminUserPassword")
        .arg("testAdminUserFullName")
        .arg("testAdminUserCountry")
        .arg("1999-01-01")
        .arg("admin")
        .output()
        .unwrap();

    // setup
    let client = Client::new();
    let response = client.post(format!("{}/login", APP_HOST))
        .json(&json!({
            "username":"testAdminUser",
            "password":"testAdminUserPassword"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: serde_json::Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION, 
        header::HeaderValue::from_str(&header_value).unwrap()
    );
    ClientBuilder::new().default_headers(headers).build().unwrap()
}
