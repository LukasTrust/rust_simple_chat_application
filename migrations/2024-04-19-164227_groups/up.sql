CREATE TABLE groups (
    id serial NOT NULL,
    name VARCHAR(100) NOT NULL,
    creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT groups_pkey PRIMARY KEY (id)
);
