use crate::backend::database::{
    db::establish_connection,
    models::UserToUserFriend,
    schema::user_to_user_friends::{self},
};
use diesel::prelude::*;

/// Create a new user friend. Returns the user friend or an error message
pub fn create_user_friend(
    user_one_id: i32,
    user_two_id: i32,
    accepted_user_one: bool,
    accepted_user_two: bool,
) -> Result<UserToUserFriend, String> {
    let user_friend = UserToUserFriend {
        user_one_id,
        user_two_id,
        accepted_user_one,
        accepted_user_two,
    };

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let user_friend = diesel::insert_into(user_to_user_friends::table)
        .values(&user_friend)
        .get_result(&mut connection)
        .map_err(|err| format!("Failed to insert user friend: {}", err))?;

    Ok(user_friend)
}

/// Accept a friend request. Returns nothing or an error message
pub fn acccepte_friend_request(user_one_id: i32, user_two_id: i32) -> Result<(), String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let result = diesel::update(user_to_user_friends::table.find((user_one_id, user_two_id)))
        .set((
            user_to_user_friends::accepted_user_one.eq(true),
            user_to_user_friends::accepted_user_two.eq(true),
        ))
        .execute(&mut connection)
        .map_err(|err| format!("Failed to update user friend: {}", err))?;

    match result {
        0 => Err("No user friend found".to_string()),
        _ => Ok(()),
    }
}

/// Find all user to user friend entries. Returns a vector of user to user friends or an error message
pub fn find_all_user_to_user_friend_entries(user_id: i32) -> Result<Vec<UserToUserFriend>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let results = user_to_user_friends::table
        .filter(
            user_to_user_friends::user_one_id
                .eq(user_id)
                .or(user_to_user_friends::user_two_id.eq(user_id)),
        )
        .load::<UserToUserFriend>(&mut connection)
        .map_err(|err| format!("Error loading user friends: {}", err))?;

    Ok(results)
}

/// Delete a friend to friend relation. Returns nothing or an error message
pub fn delete_friend_to_friend_relation(user_one_id: i32, user_two_id: i32) -> Result<(), String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let result = diesel::delete(user_to_user_friends::table.find((user_one_id, user_two_id)))
        .execute(&mut connection)
        .map_err(|err| format!("Failed to delete user friend: {}", err))?;

    match result {
        0 => Err("No user friend found".to_string()),
        _ => Ok(()),
    }
}
