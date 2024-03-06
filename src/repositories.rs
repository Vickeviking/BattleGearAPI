use crate::{models::*, rocket_routes::CacheConn, schema::*};
use crate::rocket_routes::server_error;

use serde_json::Value;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use diesel::prelude::*;
use rocket::{response::status::Custom};
use rocket_db_pools::Connection;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;

// ----------------- Seassions  -----------------
pub struct SessionRepository;

impl SessionRepository {
    pub async fn create_session(cache: &mut Connection<CacheConn>, session_id: String, user_id: i32) -> Result<(), Custom<Value>> {
        cache.set_ex::<String, i32, ()>(
            format!("sessions/{}", session_id), 
            user_id, 
            3*60*60
        )
        .await
        .map_err(|e| server_error(e.into()))
    }

}
// -----------------  User  -----------------
pub struct UserRepository;

impl UserRepository {  //CRUD operations for user

    pub async fn find_by_username(c: &mut AsyncPgConnection, username: &String) -> QueryResult<User> {
        users::table.filter(users::username.eq(username)).get_result(c).await
    }

    pub async fn find_with_roles(c: &mut AsyncPgConnection) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        //load all users into a variabel 
        let users = users::table.load::<User>(c).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c).await?
            .grouped_by(&users);
        Ok(users.into_iter().zip(result).collect())
    }

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }
    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<User>>{
        users::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user: NewUser, role_codes: Vec<String>) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(c)
            .await?;

        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepository::find_by_code(c, role_code.clone()).await {
                    NewUserRole {
                        user_id: user.user_id,
                        role_id: role.id,
                    }
                } else {
                    let new_role = NewRole {
                        code: role_code.to_owned(),
                        name: role_code.to_owned()
                    };
                    let role = RoleRepository::create(c, new_role).await?;
                    NewUserRole {
                        user_id: user.user_id,
                        role_id: role.id,
                    }
                }
            };
            //if role exists, add it to the user
            

            diesel::insert_into(users_roles::table)
                .values(&new_user_role)
                .get_result::<UserRole>(c)
                .await?;

        }
        Ok(user)

    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, user: User) -> QueryResult<User> {
        diesel::update(users::table.find(id))
            .set((
                users::username.eq(user.username),
                users::email.eq(user.email),
                users::password_hash.eq(user.password_hash),
                users::full_name.eq(user.full_name),
                users::avatar_id.eq(user.avatar_id),
                users::last_login.eq(user.last_login),
                users::is_active.eq(user.is_active),
                users::is_admin.eq(user.is_admin),
                users::timezone.eq(user.timezone),
                users::language.eq(user.language),
                users::country.eq(user.country),
                users::two_factor_auth_enabled.eq(user.two_factor_auth_enabled),
                users::last_password_change.eq(user.last_password_change),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        // delete all owned tables before deleting the user
        // as of now user owns: 
        // - user_roles, total_throphies, throphies, user_levels, currency, friendships

        // delete user roles
        diesel::delete(
            users_roles::table.filter(users_roles::user_id.eq(id))
        ).execute(c).await?;
        // delete total throphies
        diesel::delete(
            total_throphies::table.filter(total_throphies::user_id.eq(id))
        ).execute(c).await?;
        // delete throphies
        diesel::delete(
            trophies::table.filter(trophies::user_id.eq(id))
        ).execute(c).await?;
        // delete user levels
        diesel::delete(
            user_levels::table.filter(user_levels::user_id.eq(id))
        ).execute(c).await?;
        // delete currency
        diesel::delete(
            currency::table.filter(currency::user_id.eq(id))
        ).execute(c).await?;
        // delete friendships
        diesel::delete(
            friendships::table.filter(friendships::user_id.eq(id))
        ).execute(c).await?;

        diesel::delete(users::table.find(id)).execute(c).await

    }

    pub async fn delete_by_username(c: &mut AsyncPgConnection, username: &String) -> QueryResult<usize> {
        let user = users::table.filter(users::username.eq(username)).get_result::<User>(c).await?;
        Self::delete(c, user.user_id).await
    }

    pub async fn username_exists(c: &mut AsyncPgConnection, username: String) -> QueryResult<bool> {
        let user = users::table.filter(users::username.eq(username)).get_result::<User>(c).await;
        match user {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }

    pub async fn email_exists(c: &mut AsyncPgConnection, email: String) -> QueryResult<bool> {
        let user = users::table.filter(users::email.eq(email)).get_result::<User>(c).await;
        match user {
            Ok(_) => Ok(true),
            Err(_) => Ok(false)
        }
    }
    

}

// -----------------  Role  -----------------
pub struct RoleRepository;

impl RoleRepository {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }

    pub async fn find_by_code(c: &mut AsyncPgConnection, code: String) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).get_result(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let user_roles = UserRole::belonging_to(&user).get_results::<UserRole>(c).await?;
        let role_ids: Vec<i32> = user_roles.iter().map(|ur: &UserRole| ur.role_id).collect();
        
        Self::find_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(&new_role)
            .get_result(c)
            .await
    }
}


// -----------------  Image  -----------------
pub struct ImageRepository;

impl ImageRepository {  //CRUD operations for image

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Image> {
        images::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Image>>{
        images::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_image: NewImage) -> QueryResult<Image> {
        diesel::insert_into(images::table)
            .values(&new_image)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32,  image: Image) -> QueryResult<Image> {
        diesel::update(images::table.find(id))
            .set((
                images::image_url.eq(image.image_url),
                images::description.eq(image.description),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(images::table.find(id)).execute(c).await
    }
}



// -----------------  TotalThrophies  -----------------
pub struct TotalThrophiesRepository;

impl TotalThrophiesRepository {  //CRUD operations for total throphies
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<TotalThrophies> {
        total_throphies::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<TotalThrophies>>{
        total_throphies::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_total_throphies: NewTotalThrophies) -> QueryResult<TotalThrophies> {
        diesel::insert_into(total_throphies::table)
            .values(&new_total_throphies)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32,  total_throphies: TotalThrophies) -> QueryResult<TotalThrophies> {
        diesel::update(total_throphies::table.find(id))
            .set((
                total_throphies::user_id.eq(total_throphies.user_id),
                total_throphies::total.eq(total_throphies.total),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(total_throphies::table.find(id)).execute(c).await
    }

}




// -----------------  Throphies  -----------------
pub struct ThrophiesRepository; 

impl ThrophiesRepository {  //CRUD operations for throphies
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Trophy> {
        trophies::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Trophy>>{
        trophies::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_trophy: NewTrophy) -> QueryResult<Trophy> {
        diesel::insert_into(trophies::table)
            .values(&new_trophy)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, trophy: Trophy) -> QueryResult<Trophy> {
        diesel::update(trophies::table.find(id))
            .set((
                trophies::user_id.eq(trophy.user_id),
                trophies::points.eq(trophy.points),
                trophies::game_timestamp.eq(trophy.game_timestamp),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(trophies::table.find(id)).execute(c).await
    }

}




// -----------------  UserLevel  -----------------
pub struct UserLevelRepository;

impl UserLevelRepository {  //CRUD operations for user level
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<UserLevel> {
        user_levels::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<UserLevel>>{
        user_levels::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_user_level: NewUserLevel) -> QueryResult<UserLevel> {
        diesel::insert_into(user_levels::table)
            .values(&new_user_level)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32,  user_level: UserLevel) -> QueryResult<UserLevel> {
        diesel::update(user_levels::table.find(id))
            .set((
                user_levels::level.eq(user_level.level),
                user_levels::experience_points.eq(user_level.experience_points),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(user_levels::table.find(id)).execute(c).await
    }

}




// -----------------  Chat  -----------------
pub struct ChatRepository;

impl ChatRepository {  //CRUD operations for chat
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Chat> {
        chats::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Chat>>{
        chats::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_chat: NewChat) -> QueryResult<Chat> {
        diesel::insert_into(chats::table)
            .values(&new_chat)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, chat: Chat) -> QueryResult<Chat> {
        diesel::update(chats::table.find(id))
            .set((
                chats::is_read.eq(chat.is_read),
                chats::message.eq(chat.message),
                chats::receiver_id.eq(chat.receiver_id),
                chats::sender_id.eq(chat.sender_id),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(chats::table.find(id)).execute(c).await
    }

}




// -----------------  Currency  -----------------
pub struct CurrencyRepository;

impl CurrencyRepository {  //CRUD operations for currency
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Currency> {
        currency::table.find(id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Currency>>{
        currency::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_currency: NewCurrency) -> QueryResult<Currency> {
        diesel::insert_into(currency::table)
            .values(&new_currency)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, currency: Currency) -> QueryResult<Currency> {
        diesel::update(currency::table.find(id))
            .set((
                currency::amount.eq(currency.amount),
                currency::currency_type.eq(currency.currency_type),
                currency::last_updated.eq(currency.last_updated),
                currency::user_id.eq(currency.user_id),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(currency::table.find(id)).execute(c).await
    }
}



// -----------------  Friendship  -----------------
pub struct FriendshipRepository;

impl FriendshipRepository {  //CRUD operations for friendship
    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<Friendship> {
        friendships::table.find(id).get_result(c).await
    }
    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Friendship>>{
        friendships::table.limit(limit).get_results(c).await
    }

    pub async fn create(c: &mut AsyncPgConnection, new_friendship: NewFriendship) -> QueryResult<Friendship> {
        diesel::insert_into(friendships::table)
            .values(&new_friendship)
            .get_result(c)
            .await
    }

    pub async fn update(c: &mut AsyncPgConnection, id: i32, friendship: Friendship) -> QueryResult<Friendship> {
        diesel::update(friendships::table.find(id))
            .set((
                friendships::friend_id.eq(friendship.friend_id),
                friendships::friendship_date.eq(friendship.friendship_date),
                friendships::status.eq(friendship.status),
                friendships::user_id.eq(friendship.user_id),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(friendships::table.find(id)).execute(c).await
    }
}