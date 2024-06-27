use crate::backend::database::db::{establish_connection, is_strong_password};
use crate::backend::database::models::*;
use crate::backend::database::schema::users::{self};

use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;

/// Create a new user. Returns the user summary or an error message
pub fn create_user(
    first_name: &str,
    last_name: &str,
    email: &str,
    password: &str,
) -> Result<User, String> {
    let user = NewUser {
        first_name,
        last_name,
        email,
        password,
    };

    // Check if the email already exists
    if user_email_exists(user.email)? {
        return Err("Email address already in use".to_string());
    }

    let hashed_password = match hash(user.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(err) => return Err(format!("Failed to hash password: {}", err)),
    };

    // Establish a new connection each time
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    // Insert the new user record and return the inserted ID
    let id: i32 = diesel::insert_into(users::table)
        .values(NewUser {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            password: &hashed_password,
        })
        .returning(users::id)
        .get_result(&mut connection)
        .map_err(|err| format!("Failed to insert new user: {}", err))?;

    // Return the user summary
    Ok(User {
        id,
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
    })
}

/// Update a user's password. Returns nothing or an error message
pub fn update_password(user_id: i32, old_password: &str, new_password: &str) -> Result<(), String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;
    use crate::backend::database::schema::users::dsl::*;

    // Retrieve the user from the database
    let existing_user = find_user_with_password_by_id(user_id)
        .map_err(|err| format!("Error finding user: {}", err))?;

    if !is_strong_password(new_password) {
        return Err("Password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one digit, and one special character".to_string());
    }

    // Verify old password
    if !verify(old_password, &existing_user.password)
        .map_err(|err| format!("Failed to verify old password: {}", err))?
    {
        return Err("Old password does not match.".to_string());
    }

    // Hash the new password
    let hashed_password = hash(new_password, DEFAULT_COST)
        .map_err(|err| format!("Failed to hash password: {}", err))?;

    // Update the user's password
    diesel::update(users.find(user_id))
        .set(password.eq(hashed_password))
        .execute(&mut connection)
        .map_err(|err| format!("Unable to update user deletion status: {}", err))?;

    Ok(())
}

/// Update a user's email. Returns nothing or an error message
pub fn update_email(user_id: i32, new_email: &str) -> Result<(), String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;
    use crate::backend::database::schema::users::dsl::*;

    // Check if the new email already exists
    if user_email_exists(new_email)? {
        return Err("Email address already in use".to_string());
    }

    diesel::update(users.find(user_id))
        .set(email.eq(new_email))
        .execute(&mut connection)
        .map_err(|err| format!("Unable to update user deletion status: {}", err))?;

    Ok(())
}

/// Find all users. Returns a vector of users or an error message
pub fn find_all_user() -> Result<Vec<User>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let results = users::table
        .select((users::id, users::first_name, users::last_name))
        .load::<User>(&mut connection)
        .map_err(|err| format!("Error loading users: {}", err))?;

    Ok(results)
}

/// Find a user by id with password. Returns the user with password or an error message
pub fn find_user_with_password_by_id(user_id: i32) -> Result<UserPassword, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;
    use crate::backend::database::schema::users::dsl::*;

    let user_result = users
        .filter(id.eq(user_id))
        .select((id, password))
        .first::<(i32, String)>(&mut connection)
        .optional()
        .map_err(|err| format!("Error querying user: {}", err))?;

    match user_result {
        Some((_id_value, password_value)) => Ok(UserPassword {
            id: user_id,
            password: password_value,
        }),
        None => Err(format!("User with id {} not found", user_id)),
    }
}

/// Find users by ids. Returns a vector of users or an error message
pub fn find_users_by_ids(user_ids: Vec<i32>) -> Result<Vec<User>, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;
    use crate::backend::database::schema::users::dsl::*;

    let results = users
        .filter(id.eq_any(user_ids))
        .select((id, first_name, last_name))
        .load::<User>(&mut connection)
        .map_err(|err| format!("Error loading users: {}", err))?;

    Ok(results)
}

/// Find a user by email. Returns the user or an error message
pub fn find_user_by_email(email_param: &str) -> Result<User, String> {
    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;
    use crate::backend::database::schema::users::dsl::*;

    let user_result = users
        .filter(email.eq(email_param))
        .select((id, first_name, last_name))
        .first::<(i32, String, String)>(&mut connection)
        .optional()
        .map_err(|err| format!("Error querying user: {}", err))?;

    match user_result {
        Some((user_id, first_name_value, last_name_value)) => Ok(User {
            id: user_id,
            first_name: first_name_value,
            last_name: last_name_value,
        }),
        None => Err(format!("User with Email {} not found", email_param)),
    }
}

/// Check if a user with the given email exists. Returns true if the email exists, false otherwise
pub fn user_email_exists(email_param: &str) -> Result<bool, String> {
    use crate::backend::database::schema::users::dsl::*;

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let other_email = users
        .filter(email.eq(email_param))
        .select(email)
        .first::<String>(&mut connection)
        .optional()
        .map_err(|err| format!("Error querying user: {}", err));

    match other_email {
        Ok(Some(_)) => Ok(true), // Email exists
        Ok(None) => Ok(false),   // Email doesn't exist
        Err(_) => Ok(false),     // Email doesn't exist
    }
}

/// Delete a user. Returns nothing or an error message
pub fn delete_user(user_id: i32) -> Result<(), String> {
    use crate::backend::database::schema::users::dsl::*;

    let mut connection =
        establish_connection().map_err(|err| format!("Failed to establish connection: {}", err))?;

    let target = users.filter(id.eq(user_id));
    diesel::delete(target)
        .execute(&mut connection)
        .map_err(|err| format!("Unable to delete user {}: {}", user_id, err))?;

    Ok(())
}
