table! {
    account_permissions (account_id, permission_id) {
        account_id -> Uuid,
        permission_id -> Uuid,
    }
}

table! {
    account_roles (account_id, role_id) {
        account_id -> Uuid,
        role_id -> Uuid,
    }
}

table! {
    accounts (id) {
        id -> Uuid,
        email -> Varchar,
        username -> Varchar,
        password_type -> Int2,
        password_hash -> Bytea,
        password_salt -> Bytea,
        auth_token -> Nullable<Varchar>,
        email_verified_at -> Nullable<Timestamptz>,
        avatar -> Nullable<Uuid>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    permissions (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Nullable<Varchar>,
        description -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    role_permissions (role_id, permission_id) {
        role_id -> Uuid,
        permission_id -> Uuid,
    }
}

table! {
    roles (id) {
        id -> Uuid,
        name -> Varchar,
        label -> Nullable<Varchar>,
        description -> Nullable<Text>,
        is_super -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(account_permissions -> accounts (account_id));
joinable!(account_permissions -> permissions (permission_id));
joinable!(account_roles -> accounts (account_id));
joinable!(account_roles -> roles (role_id));
joinable!(role_permissions -> permissions (permission_id));
joinable!(role_permissions -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    account_permissions,
    account_roles,
    accounts,
    permissions,
    role_permissions,
    roles,
);
