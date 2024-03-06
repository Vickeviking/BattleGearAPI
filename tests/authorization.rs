use std::process::Command;
use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
mod common;

#[test]
fn test_login() {
    // create user through cli
    let output = Command::new("cargo")
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
    println!("{:?}", output);

    // setup
    let client = Client::new();

    // test authorized
    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"testAdminUser",
            "password":"testAdminUserPassword"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // test that token exists and that it is of correct lenght 128
    let json: serde_json::Value = response.json().unwrap();
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    // test unauthorized
    let response = client.post(format!("{}/login", common::APP_HOST))
        .json(&json!({
            "username":"testAdminUser",
            "password":"testAdminUserPasswordWrong"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // clean up
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("delete_by_username")
        .arg("testAdminUser")
        .output()
        .unwrap();
    println!("{:?}", output);

    
    
}