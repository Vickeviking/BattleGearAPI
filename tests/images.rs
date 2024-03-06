use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::serde_json::json;
use serde_json::Value;

mod common;
use common::APP_HOST;


/*          Date: 2024-02-21
    Works perfectly, all endpoints are tested and working✅
    Clean up works, and all users are deleted after each test✅

    Side note: The images depend on users, while they store user_id, 
    1. so images are deleted before users, to avoid foreign key constraint error
    2. images are created after users, to avoid foreign key constraint error
*/

#[test]
fn test_endpont_protected() {
    let client = Client::new();
    let response = client.get(format!("{}/images", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}


fn create_test_image(client: &Client) -> Value {
    let response = client.post(format!("{}/images", APP_HOST))
        .json(&json!({
            "image_url":"https://www.google.com",
            "description":"hello"
        }))
        .send()
        .unwrap();
    let status = response.status();
    assert_eq!(status, StatusCode::CREATED);
    let images: Value = response.json().unwrap();
    println!("{:#?}", images);
    images
}

fn delete_test_image(client: &Client, image: Value) {
    let response = client.delete(format!("{}/images/{}", APP_HOST, image["image_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[test]
fn test_get_images() {
    //setup
    let client = common::get_client_with_logged_in_admin();
    let image1: Value = create_test_image(&client);
    let image2: Value = create_test_image(&client);

    // test
    let response = client.get(format!("{}/images", APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&image1));
    assert!(json.as_array().unwrap().contains(&image2));

    // clean up
    delete_test_image(&client, image1);
    delete_test_image(&client, image2);
}

#[test]
fn test_view_images() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let image: Value = create_test_image(&client);

    // test
    let response = client.get(format!("{}/images/{}", APP_HOST, image["image_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let image: Value = response.json().unwrap();
    assert_eq!(image, json!({
        "image_id": image["image_id"],
        "image_url":"https://www.google.com",
        "description":"hello",
        "upload_date": image["upload_date"]
    }));

    // clean up
    delete_test_image(&client, image);
}

#[test]
fn test_create_images() {
    // test create images
    let client = common::get_client_with_logged_in_admin();

    let response = client.post(format!("{}/images", APP_HOST))
        .json(&json!({
            "image_url":"https://www.google.com",
            "description":"hello"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    // confirm data correctness
    let image: Value = response.json().unwrap();
    assert_eq!(image, json!({
        "image_id": image["image_id"],
        "image_url":"https://www.google.com",
        "description":"hello",
        "upload_date": image["upload_date"]
    }));

    // clean up
    delete_test_image(&client, image);
}


#[test]
fn test_update_images() {
    // setup
    let client = common::get_client_with_logged_in_admin();
    let image: Value = create_test_image(&client);
    // test
    let response = client.put(format!("{}/images/{}", APP_HOST, image["image_id"]))
        .json(&json!({
            "image_url":"https://www.facebook.com",
            "description":"hello",
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    // check if user is updated
    let image: Value = response.json().unwrap();
    assert_eq!(image, json!({
        "image_id": image["image_id"],
        "image_url":"https://www.facebook.com",
        "description":"hello",
        "upload_date": image["upload_date"]
    }));

    // clean up
    delete_test_image(&client, image);

}

#[test]
fn test_delete_images() {
    let client = common::get_client_with_logged_in_admin();
    let image: Value = create_test_image(&client);

    let response = client.delete(format!("{}/images/{}", APP_HOST, image["image_id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    // cleans up itself
} 