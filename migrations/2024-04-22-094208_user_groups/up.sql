CREATE TABLE user_to_groups (
    user_id INT NOT NULL,
    group_id INT NOT NULL,
    accepted_invite BOOLEAN DEFAULT FALSE,
    CONSTRAINT user_to_groups_pkey PRIMARY KEY (user_id, group_id),
    CONSTRAINT user_to_groups_user_id_fkey FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT user_to_groups_group_id_fkey FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE CASCADE
);
