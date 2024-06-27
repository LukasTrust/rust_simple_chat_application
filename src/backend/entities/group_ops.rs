use crate::backend::database::db::establish_connection;
use crate::backend::database::models::*;
use crate::backend::database::schema::groups::{self};

use diesel::prelude::*;

/// Create a new group. Returns the group or an error message
pub fn create_group(name: &str) -> Result<Group, String> {
    let new_group = NewGroup { name };

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let id: i32 = diesel::insert_into(groups::table)
        .values(&new_group)
        .returning(groups::id)
        .get_result(&mut connection)
        .map_err(|err| format!("Failed to insert new group: {}", err))?;

    Ok(Group {
        id,
        name: new_group.name.to_string(),
    })
}

/// Find groups by their IDs. Returns the groups or an error message
pub fn find_groups_by_ids(group_ids: Vec<i32>) -> Result<Vec<Group>, String> {
    use crate::backend::database::schema::groups::dsl::*;

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let mut result_groups = Vec::new();
    for group_id in group_ids {
        match groups
            .find(group_id)
            .select((id, name))
            .first::<Group>(&mut connection)
            .map_err(|err| format!("Failed to find group {}: {}", group_id, err))
        {
            Ok(group) => result_groups.push(group),
            Err(err) => return Err(format!("Unable to find group {}: {}", group_id, err)),
        }
    }

    Ok(result_groups)
}

/// Delete a group. Returns nothing or an error message
pub fn delete_group(group_id: i32) -> Result<(), String> {
    use crate::backend::database::schema::groups::dsl::*;

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let target = groups.filter(id.eq(group_id));
    diesel::delete(target)
        .execute(&mut connection)
        .map_err(|err| format!("Error deleting group {}: {}", group_id, err))?;

    Ok(())
}
