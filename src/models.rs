use chrono::{naive::NaiveDateTime, NaiveDate};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::schema::*;
// -----------------  User  -----------------
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Identifiable)]
#[diesel(primary_key(user_id))]
pub struct User {
    #[serde(skip_deserializing)]
    pub user_id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String, // changed from Option<String> to String
    pub avatar_id: Option<i32>,
    #[serde(skip_deserializing)]
    pub registration_date: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub is_active: Option<bool>,
    #[serde(skip_deserializing)]
    pub is_admin: Option<bool>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: NaiveDate, // changed from Option<NaiveDate> to NaiveDate
    pub two_factor_auth_enabled: Option<bool>,
    #[serde(skip_deserializing)]
    pub last_password_change: Option<NaiveDateTime>,
}


#[derive(Insertable, Deserialize)]
#[diesel(table_name=users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub country: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
}

// ----------------- Role  -----------------
#[derive(Queryable, Debug)]
pub struct Role {
    pub id: i32,
    pub code: String,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name=roles)]
pub struct NewRole {
    pub code: String,
    pub name: String,
}

// ----------------- UserRoles  -----------------
#[derive(Queryable, Associations, Identifiable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name=users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name=users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}


// -----------------  Image  -----------------
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

// -----------------  TotalThrophies  -----------------

#[derive(Queryable, Serialize, Deserialize, Debug)]

#[diesel(belongs_to(User))]
pub struct TotalThrophies {
    #[serde(skip_deserializing)]
    pub total_throphies_id: i32,
    pub user_id: Option<i32>,
    pub total: Option<i32>,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name=total_throphies)]

pub struct NewTotalThrophies {
    pub user_id: Option<i32>,
    pub total: Option<i32>
}

// -----------------  Trophies  -----------------
#[derive(Queryable, Serialize, Deserialize, Debug)]

#[diesel(belongs_to(User))]
pub struct Trophy {
    #[serde(skip_deserializing)]
    pub trophy_id: i32,
    pub user_id: Option<i32>,
    pub points: Option<i32>,
    #[serde(skip_deserializing)]
    pub game_timestamp: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=trophies)]
pub struct NewTrophy {
    pub user_id: Option<i32>,
    pub points: Option<i32>
}

// -----------------  UserLevel  -----------------
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]

#[diesel(belongs_to(User))]
pub struct UserLevel {
    #[serde(skip_deserializing)]
    pub user_level_id: i32,
    pub user_id: Option<i32>,
    pub level: Option<i32>,
    pub experience_points: Option<i32>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=user_levels)]
pub struct NewUserLevel {
    pub user_id: Option<i32>,
    pub level: Option<i32>,
    pub experience_points: Option<i32>,
}

// -----------------  Chat  -----------------
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
pub struct Chat {
    #[serde(skip_deserializing)]
    pub chat_id: i32,
    pub sender_id: Option<i32>,
    pub receiver_id: Option<i32>,
    pub message: Option<String>,
    #[serde(skip_deserializing)]
    pub timestamp: Option<NaiveDateTime>,
    pub is_read: Option<bool>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=chats)]
pub struct NewChat {
    pub sender_id: Option<i32>,
    pub receiver_id: Option<i32>,
    pub message: Option<String>,
}

// -----------------  Currency  -----------------
#[derive(Queryable, Serialize, Deserialize, Debug)]

#[diesel(belongs_to(User))]
pub struct Currency {
    #[serde(skip_deserializing)]
    pub currency_id: i32,
    pub user_id: Option<i32>,
    pub currency_type: Option<String>,
    pub amount: Option<i32>,
    #[serde(skip_deserializing)]
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=currency)]
pub struct NewCurrency {
    pub user_id: Option<i32>,
    pub currency_type: Option<String>,
    pub amount: Option<i32>,
}


// -----------------  Friendship  -----------------
#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]

#[diesel(belongs_to(User))]
pub struct Friendship {
    #[serde(skip_deserializing)]
    pub friendship_id: i32,
    pub user_id: Option<i32>,
    pub friend_id: Option<i32>,
    pub status: String,
    #[serde(skip_deserializing)]
    pub friendship_date: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name=friendships)]
pub struct NewFriendship {
    pub user_id: Option<i32>,
    pub friend_id: Option<i32>,
    pub status: String,
}
