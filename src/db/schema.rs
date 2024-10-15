// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "LoginStatus"))]
    pub struct LoginStatus;
}

diesel::table! {
    user (id) {
        id -> Uuid,
        #[max_length = 64]
        firstname -> Varchar,
        #[max_length = 64]
        lastname -> Varchar,
        #[max_length = 64]
        email -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        is_active -> Bool,
        #[max_length = 20]
        phone_number -> Nullable<Varchar>,
        birthdate -> Nullable<Date>,
        #[max_length = 100]
        city -> Nullable<Varchar>,
        #[max_length = 100]
        country -> Nullable<Varchar>,
        address -> Nullable<Text>,
        is_deleted -> Bool,
        is_verified -> Bool,
        picture -> Nullable<Text>,
        created_at -> Timestamp,
        created_by -> Nullable<Uuid>,
        updated_at -> Nullable<Timestamp>,
        updated_by -> Nullable<Uuid>,
        deleted_at -> Nullable<Timestamp>,
        deleted_by -> Nullable<Uuid>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::LoginStatus;

    user_login_history (id) {
        id -> Uuid,
        user_id -> Uuid,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        browser_info -> Nullable<Text>,
        device_info -> Nullable<Text>,
        #[max_length = 50]
        operating_system -> Nullable<Varchar>,
        status -> LoginStatus,
        failure_reason -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(user_login_history -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    user,
    user_login_history,
);
