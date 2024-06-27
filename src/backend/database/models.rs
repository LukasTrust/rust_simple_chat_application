use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::Insertable, query_builder::AsChangeset};

use super::schema::{
    group_messages, groups, user_messages, user_to_groups, user_to_user_friends, users,
};
use std::fmt;

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = users)]
/// NewUser is a struct that represents a new user that can be inserted into the database
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Clone, PartialEq, Eq)]
/// User is a struct that represents a user in the database
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}", self.first_name, self.last_name)
    }
}

#[derive(Debug, Insertable)]
#[diesel(table_name = user_messages)]
/// NewUserMessage is a struct that represents a new message that can be inserted into the database
pub struct NewUserMessage<'a> {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Clone)]
/// UserMessage is a struct that represents a message in the database
pub struct UserMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub send_date: NaiveDateTime,
}

#[derive(Debug, Insertable, Queryable, AsChangeset, Clone)]
#[diesel(table_name = user_to_user_friends)]
/// UserToUserFriend is a struct that represents a friendship between two users in the database
pub struct UserToUserFriend {
    pub user_one_id: i32,
    pub user_two_id: i32,
    pub accepted_user_one: bool,
    pub accepted_user_two: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = groups)]
/// NewGroup is a struct that represents a new group that can be inserted into the database
pub struct NewGroup<'a> {
    pub name: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Clone, PartialEq, Eq)]
/// Group is a struct that represents a group in the database
pub struct Group {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Insertable, Queryable, AsChangeset)]
#[diesel(table_name = user_to_groups)]
/// UserGroup is a struct that represents a user's membership in a group in the database
pub struct UserToGroup {
    pub user_id: i32,
    pub group_id: i32,
    pub accepted_invite: bool,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = group_messages)]
/// NewGroupMessage is a struct that represents a new group message that can be inserted into the database
pub struct NewGroupMessage<'a> {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: &'a str,
}

#[derive(Debug, Queryable, AsChangeset, Clone, PartialEq, Eq)]
/// GroupMessage is a struct that represents a group message in the database
pub struct GroupMessage {
    pub sender_id: i32,
    pub receiver_id: i32,
    pub message: String,
    pub send_date: NaiveDateTime,
}

#[derive(Debug)]
/// UserPassword is a struct that represents a user's password in the database
pub struct UserPassword {
    pub id: i32,
    pub password: String,
}
