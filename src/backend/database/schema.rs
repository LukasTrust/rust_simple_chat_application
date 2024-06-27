// @generated automatically by Diesel CLI.

use diesel::table;

table! {
    group_messages (sender_id, receiver_id, send_date) {
        sender_id -> Int4,
        receiver_id -> Int4,
        message -> Text,
        send_date -> Timestamp,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Varchar,
        creation_date -> Timestamp,
    }
}

table! {
    user_to_groups (user_id, group_id, accepted_invite) {
        user_id -> Int4,
        group_id -> Int4,
        accepted_invite -> Bool,
    }
}

table! {
    user_messages (sender_id, receiver_id, send_date) {
        sender_id -> Int4,
        receiver_id -> Int4,
        message -> Text,
        send_date -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

table! {
    user_to_user_friends (user_one_id, user_two_id) {
        user_one_id -> Int4,
        user_two_id -> Int4,
        accepted_user_one -> Bool,
        accepted_user_two -> Bool,
    }
}
