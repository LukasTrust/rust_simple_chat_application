CREATE TABLE group_messages (
    sender_id INT NOT NULL,
    receiver_id INT NOT NULL,
    message TEXT NOT NULL,
    send_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT group_messages_pkey PRIMARY KEY (sender_id, receiver_id, send_date),
    CONSTRAINT group_messages_sender_id_fkey FOREIGN KEY (sender_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT group_messages_receiver_id_fkey FOREIGN KEY (receiver_id) REFERENCES groups(id) ON DELETE CASCADE
);
