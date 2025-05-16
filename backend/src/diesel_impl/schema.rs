// @generated automatically by Diesel CLI.

diesel::table! {
    actions (id) {
        id -> Integer,
        #[max_length = 50]
        key -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    permissions (id) {
        id -> Integer,
        #[max_length = 50]
        resource -> Varchar,
        #[max_length = 50]
        action -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    role_permissions (id) {
        id -> Integer,
        role_id -> Integer,
        permission_id -> Integer,
    }
}

diesel::table! {
    roles (id) {
        id -> Integer,
        #[max_length = 50]
        name -> Varchar,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    tokens (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 255]
        token -> Varchar,
        #[max_length = 50]
        device_id -> Nullable<Varchar>,
        device_info -> Nullable<Text>,
        #[max_length = 45]
        ip_address -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
        last_used_at -> Nullable<Datetime>,
        expires_at -> Datetime,
        revoked -> Nullable<Bool>,
    }
}

diesel::table! {
    user_roles (id) {
        id -> Integer,
        user_id -> Integer,
        role_id -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        employee_id -> Nullable<Integer>,
        #[max_length = 50]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        is_active -> Nullable<Bool>,
        created_at -> Nullable<Datetime>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    actions,
    permissions,
    role_permissions,
    roles,
    tokens,
    user_roles,
    users,
);
