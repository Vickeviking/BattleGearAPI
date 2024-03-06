// @generated automatically by Diesel CLI.

diesel::table! {
    chats (chat_id) {
        chat_id -> Int4,
        sender_id -> Nullable<Int4>,
        receiver_id -> Nullable<Int4>,
        #[max_length = 1000]
        message -> Nullable<Varchar>,
        timestamp -> Nullable<Timestamptz>,
        is_read -> Nullable<Bool>,
    }
}

diesel::table! {
    currency (currency_id) {
        currency_id -> Int4,
        user_id -> Nullable<Int4>,
        #[max_length = 50]
        currency_type -> Nullable<Varchar>,
        amount -> Nullable<Int4>,
        last_updated -> Nullable<Timestamp>,
    }
}

diesel::table! {
    friendships (friendship_id) {
        friendship_id -> Int4,
        user_id -> Nullable<Int4>,
        friend_id -> Nullable<Int4>,
        #[max_length = 50]
        status -> Varchar,
        friendship_date -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    images (image_id) {
        image_id -> Int4,
        #[max_length = 255]
        image_url -> Varchar,
        description -> Nullable<Text>,
        upload_date -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 128]
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    total_throphies (total_throphies_id) {
        total_throphies_id -> Int4,
        user_id -> Nullable<Int4>,
        total -> Nullable<Int4>,
    }
}

diesel::table! {
    trophies (trophy_id) {
        trophy_id -> Int4,
        user_id -> Nullable<Int4>,
        points -> Nullable<Int4>,
        game_timestamp -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    user_levels (user_level_id) {
        user_level_id -> Int4,
        user_id -> Nullable<Int4>,
        level -> Nullable<Int4>,
        experience_points -> Nullable<Int4>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 254]
        email -> Varchar,
        #[max_length = 128]
        password_hash -> Varchar,
        #[max_length = 255]
        full_name -> Varchar,
        avatar_id -> Nullable<Int4>,
        registration_date -> Nullable<Timestamptz>,
        last_login -> Nullable<Timestamptz>,
        is_active -> Nullable<Bool>,
        is_admin -> Nullable<Bool>,
        #[max_length = 50]
        timezone -> Nullable<Varchar>,
        #[max_length = 50]
        language -> Nullable<Varchar>,
        #[max_length = 50]
        country -> Nullable<Varchar>,
        date_of_birth -> Date,
        two_factor_auth_enabled -> Nullable<Bool>,
        last_password_change -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

diesel::joinable!(currency -> users (user_id));
diesel::joinable!(total_throphies -> users (user_id));
diesel::joinable!(trophies -> users (user_id));
diesel::joinable!(user_levels -> users (user_id));
diesel::joinable!(users -> images (avatar_id));
diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    chats,
    currency,
    friendships,
    images,
    roles,
    total_throphies,
    trophies,
    user_levels,
    users,
    users_roles,
);
