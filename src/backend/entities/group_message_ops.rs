use crate::backend::database::db::establish_connection;
use crate::backend::database::models::*;
use crate::backend::database::schema::group_messages::{self};

use diesel::prelude::*;

/// Create a new group message. Returns the group message or an error message
pub fn create_group_message(
    sender_id: i32,
    receiver_id: i32,
    message: &str,
) -> Result<GroupMessage, String> {
    let new_group_message = NewGroupMessage {
        sender_id,
        receiver_id,
        message,
    };

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    diesel::insert_into(group_messages::table)
        .values(&new_group_message)
        .execute(&mut connection)
        .map_err(|err| format!("Failed to insert group message: {}", err))?;

    let group_message_summary = GroupMessage {
        sender_id: new_group_message.sender_id,
        receiver_id: new_group_message.receiver_id,
        message: new_group_message.message.to_owned(),
        send_date: chrono::Local::now().naive_local(),
    };

    Ok(group_message_summary)
}

/// Find all group messages. Returns a vector of group messages or an error message
pub fn find_all_messages_of_group(group_id: i32) -> Result<Vec<GroupMessage>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let messages = group_messages::table
        .filter(group_messages::receiver_id.eq(group_id))
        .load::<GroupMessage>(&mut connection)
        .map_err(|err| format!("Failed to load group messages: {}", err))?;

    Ok(messages)
}

/// Delete all group messages. Returns the number of deleted messages or an error message
pub fn delete_group_messages(sender_id: i32, receiver_id: i32) -> Result<usize, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let num_deleted = diesel::delete(
        group_messages::table
            .filter(group_messages::sender_id.eq(sender_id))
            .filter(group_messages::receiver_id.eq(receiver_id)),
    )
    .execute(&mut connection)
    .map_err(|err| format!("Failed to delete group messages: {}", err))?;

    Ok(num_deleted)
}
