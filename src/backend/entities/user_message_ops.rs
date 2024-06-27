use crate::backend::database::db::establish_connection;
use crate::backend::database::models::*;
use crate::backend::database::schema::user_messages::{self};

use diesel::prelude::*;

/// Create a new user message. Returns the user message summary or an error message
pub fn create_user_message(
    sender_id: i32,
    receiver_id: i32,
    message: &str,
) -> Result<UserMessage, String> {
    let new_user_message = NewUserMessage {
        sender_id,
        receiver_id,
        message,
    };

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    diesel::insert_into(user_messages::table)
        .values(&new_user_message)
        .execute(&mut connection)
        .map_err(|err| format!("Failed to insert new user message: {}", err))?;

    let group_message_summary = UserMessage {
        sender_id: new_user_message.sender_id,
        receiver_id: new_user_message.receiver_id,
        message: new_user_message.message.to_string(),
        send_date: chrono::Local::now().naive_local(),
    };

    Ok(group_message_summary)
}

/// Find all messages between two users. Returns a list of user messages or an error message
pub fn find_all_messages_between_users(
    user1_id: i32,
    user2_id: i32,
) -> Result<Vec<UserMessage>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let results = user_messages::table
        .filter(
            (user_messages::sender_id
                .eq(user1_id)
                .and(user_messages::receiver_id.eq(user2_id)))
            .or(user_messages::sender_id
                .eq(user2_id)
                .and(user_messages::receiver_id.eq(user1_id))),
        )
        .select((
            user_messages::sender_id,
            user_messages::receiver_id,
            user_messages::message,
            user_messages::send_date,
        ))
        .load::<UserMessage>(&mut connection)
        .map_err(|err| format!("Error loading messages: {}", err))?;

    Ok(results)
}

/// Delete a user message. Returns nothing or an error message
pub fn delete_user_message(other_sender_id: i32, other_receiver_id: i32) -> Result<(), String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    diesel::delete(
        user_messages::table.filter(
            user_messages::sender_id
                .eq(other_sender_id)
                .and(user_messages::receiver_id.eq(other_receiver_id)),
        ),
    )
    .execute(&mut connection)
    .map_err(|err| format!("Failed to delete user message: {}", err))?;

    Ok(())
}
