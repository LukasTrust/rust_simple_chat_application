// @generated automatically by Diesel CLI.

diesel::table! {
    group_messages (sender_id, receiver_id, send_date) {
        sender_id -> Int4,
        receiver_id -> Int4,
        message -> Text,
        send_date -> Timestamp,
    }
}

diesel::table! {
    groups (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        creation_date -> Timestamp,
    }
}

diesel::table! {
    user_messages (sender_id, receiver_id, send_date) {
        sender_id -> Int4,
        receiver_id -> Int4,
        message -> Text,
        send_date -> Timestamp,
    }
}

diesel::table! {
    user_to_groups (user_id, group_id) {
        user_id -> Int4,
        group_id -> Int4,
        accepted_invite -> Nullable<Bool>,
    }
}

diesel::table! {
    user_to_user_friends (user_one_id, user_two_id) {
        user_one_id -> Int4,
        user_two_id -> Int4,
        accepted_user_one -> Nullable<Bool>,
        accepted_user_two -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
    }
}

diesel::joinable!(group_messages -> groups (receiver_id));
diesel::joinable!(group_messages -> users (sender_id));
diesel::joinable!(user_to_groups -> groups (group_id));
diesel::joinable!(user_to_groups -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    group_messages,
    groups,
    user_messages,
    user_to_groups,
    user_to_user_friends,
    users,
);
