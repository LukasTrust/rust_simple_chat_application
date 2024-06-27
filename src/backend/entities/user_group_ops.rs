use crate::backend::database::db::establish_connection;
use crate::backend::database::models::*;
use crate::backend::database::schema::user_to_groups;

use diesel::prelude::*;

/// Create a new user group. Returns the user group or an error message
pub fn create_user_group(
    user_id: i32,
    group_id: i32,
    accepted_invite: bool,
) -> Result<UserToGroup, String> {
    let user_group = UserToGroup {
        user_id,
        group_id,
        accepted_invite,
    };

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    diesel::insert_into(user_to_groups::table)
        .values(&user_group)
        .execute(&mut connection)
        .map_err(|err| format!("Failed to insert new user group: {}", err))?;

    let user_group = UserToGroup {
        user_id: user_group.user_id,
        group_id: user_group.group_id,
        accepted_invite: false,
    };

    Ok(user_group)
}

/// Update a user group. Returns the user group or an error message
pub fn update_user_group(
    user_id: i32,
    group_id: i32,
    accepted_invite: bool,
) -> Result<UserToGroup, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    diesel::update(
        user_to_groups::table
            .filter(user_to_groups::user_id.eq(user_id))
            .filter(user_to_groups::group_id.eq(group_id)),
    )
    .set(user_to_groups::accepted_invite.eq(accepted_invite))
    .execute(&mut connection)
    .map_err(|err| format!("Failed to update user group: {}", err))?;

    let user_group = UserToGroup {
        user_id,
        group_id,
        accepted_invite,
    };

    Ok(user_group)
}

/// Find all user groups of user. Returns a vector of user groups or an error message
pub fn find_all_user_groups_of_user(user_id: i32) -> Result<Vec<UserToGroup>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let results = user_to_groups::table
        .filter(user_to_groups::user_id.eq(user_id))
        .select((
            user_to_groups::user_id,
            user_to_groups::group_id,
            user_to_groups::accepted_invite,
        ))
        .load::<UserToGroup>(&mut connection)
        .map_err(|err| format!("Error loading user groups: {}", err))?;

    Ok(results)
}

/// Find all user groups of group. Returns a vector of user groups or an error message
pub fn find_all_user_groups_of_group(group_id: i32) -> Result<Vec<UserToGroup>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let results = user_to_groups::table
        .filter(user_to_groups::group_id.eq(group_id))
        .select((
            user_to_groups::user_id,
            user_to_groups::group_id,
            user_to_groups::accepted_invite,
        ))
        .load::<UserToGroup>(&mut connection)
        .map_err(|err| format!("Error loading user groups: {}", err))?;

    Ok(results)
}

/// Delete a user group. Returns the number of user groups deleted or an error message
pub fn delete_user_group(user_id: i32, group_id: i32) -> Result<usize, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let num_deleted = diesel::delete(
        user_to_groups::table
            .filter(user_to_groups::user_id.eq(user_id))
            .filter(user_to_groups::group_id.eq(group_id)),
    )
    .execute(&mut connection)
    .map_err(|err| format!("Failed to delete user group: {}", err))?;

    Ok(num_deleted)
}
