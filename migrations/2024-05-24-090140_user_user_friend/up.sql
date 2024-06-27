CREATE TABLE user_to_user_friends (
    user_one_id INT NOT NULL,
    user_two_id INT NOT NULL,
    accepted_user_one BOOLEAN DEFAULT FALSE,
    accepted_user_two BOOLEAN DEFAULT FALSE,
    CONSTRAINT user_to_user_friends_pkey PRIMARY KEY (user_one_id, user_two_id),
    CONSTRAINT user_to_user_friends_user_one_id_fkey FOREIGN KEY (user_one_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT user_to_user_friends_user_two_id_fkey FOREIGN KEY (user_two_id) REFERENCES users(id) ON DELETE CASCADE
);
