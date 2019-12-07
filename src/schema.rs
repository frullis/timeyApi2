table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

table! {
    sessions (id) {
        id -> Integer,
        user_id -> Integer,
        secret -> Varchar,
        created_at -> Datetime,
        expire_at -> Datetime,
    }
}


table! {
    projects (id) {
        id -> Integer,
        name -> Nullable<Varchar>,
        created_at -> Nullable<Datetime>,
    }
}

table! {
    time_activity (id) {
        id -> Integer,
        time_start -> Varchar,
        duration -> Nullable<Integer>,
        time_stop -> Nullable<Varchar>,
        project_id -> Nullable<Integer>,
        agent -> Nullable<Varchar>,
    }
}

table! {
    users (id) {
        id -> Integer,
        email -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}


allow_tables_to_appear_in_same_query!(
    sessions,
    users,
    posts,
    projects,
    time_activity,
);
