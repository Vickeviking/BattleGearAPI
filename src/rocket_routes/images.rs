use crate::models::{NewImage, Image, User};
use crate::repositories::ImageRepository;
use crate::rocket_routes::{DbConn, server_error};
use rocket::{response::status::Custom, serde::json::Json, response::status::NoContent};
use rocket::http::Status;
use rocket_db_pools::Connection;
use rocket::serde::json::{json, Value};


/*  TESTED  , 
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Image {
    #[serde(skip_deserializing)]
    pub image_id: i32,
    pub image_url: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    pub upload_date: Option<NaiveDateTime>,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name=images)]
pub struct NewImage {
    pub image_url: String,
    pub description: Option<String>
}

*/

//------------- get endpoint -------------
//multi
#[rocket::get("/images")]
pub async fn get_images(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    ImageRepository::find_multiple(&mut db, 100).await
        .map(|images| json!(images))
        .map_err(|e| server_error(e.into())) // hiding error from client for now
}   
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/images
*/

//single image
#[rocket::get("/images/<id>")]
pub async fn view_image(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<Value, Custom<Value>> {
    ImageRepository::find(&mut db, id).await
        .map(|image| json!(image))
        .map_err(|e| server_error(e.into()))
}
/*
    Test Endpoint with: 
    docker-compose exec app curl 127.0.0.1:8000/images/1
*/

//------------- create endpoint -------------
#[rocket::post("/images", format="json", data="<new_image>")]
pub async fn create_image(mut db: Connection<DbConn>, new_image: Json<NewImage>, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    ImageRepository::create(&mut db, new_image.into_inner()).await
        .map(|image| Custom(Status::Created, json!(image)))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/images -H 'Content-type: application/json' 
  -d '{"image_url":"https://www.google.com","description":"hello"}'
*/

//------------- update endpoint -------------
#[rocket::put("/images/<id>", format="json", data="<image>")]
pub async fn update_image(mut db: Connection<DbConn>, id: i32, image: Json<Image>, _user: User) -> Result<Value, Custom<Value>> {
    ImageRepository::update(&mut db, id, image.into_inner()).await
        .map(|image| json!(image))
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with:  
  docker-compose exec app curl 127.0.0.1:8000/images/1 -X PUT -H 'Content-type: application/json' 
  -d '{"image_url":"https://www.google.com","description":"hello"}'
*/

//------------- delete endpoint -------------
#[rocket::delete("/images/<id>")]
pub async fn delete_image(mut db: Connection<DbConn>, id: i32, _user: User) -> Result<NoContent, Custom<Value>> {
    ImageRepository::delete(&mut db, id).await
        .map(|_|  NoContent)
        .map_err(|e| server_error(e.into()))
}
/* Test Endpoint with: 
  docker-compose exec app curl 127.0.0.1:8000/images/1 -X DELETE 
*/