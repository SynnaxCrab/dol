table! {
    discoveries (id) {
        id -> Int4,
        uid -> Int4,
        category -> Varchar,
        name -> Varchar,
        description -> Text,
        level -> Nullable<Int4>,
        points -> Nullable<Int4>,
        era -> Nullable<Varchar>,
        difficulty -> Nullable<Int4>,
        exp -> Nullable<Int4>,
        note -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        password_digest -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    discoveries,
    users,
);
