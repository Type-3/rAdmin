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
        roles -> Array<Varchar>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    roles (name) {
        name -> Varchar,
        label -> Nullable<Varchar>,
        description -> Nullable<Text>,
        is_super -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(accounts, roles);