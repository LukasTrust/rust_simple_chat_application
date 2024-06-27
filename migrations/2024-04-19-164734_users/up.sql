CREATE TABLE users (
    id serial NOT NULL,
    first_name VARCHAR(50) NOT NULL,
    last_name VARCHAR(50) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password VARCHAR(100) NOT NULL,
    CONSTRAINT users_key PRIMARY KEY (id)
);
