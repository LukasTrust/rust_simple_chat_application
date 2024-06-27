#[cfg(test)]
mod tests {
    use secse24_group08::backend::database::models::User;

    #[test]
    fn display_user_test() {
        let user = User {
            id: 1,
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
        };
        assert_eq!(format!("{}", user), "John, Doe");
    }
}
