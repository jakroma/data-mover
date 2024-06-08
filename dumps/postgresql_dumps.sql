CREATE TABLE user_types (
    id SERIAL PRIMARY KEY,
    type_name VARCHAR(50) NOT NULL
);

INSERT INTO user_types (type_name) VALUES ('Admin'), ('Standard');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL,
    user_type_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_type_id) REFERENCES user_types(id)
);

INSERT INTO users (username, email, user_type_id) VALUES 
('admin_user1', 'admin1@example.com', 1),
('admin_user2', 'admin2@example.com', 1),
('standard_user1', 'standard1@example.com', 2),
('standard_user2', 'standard2@example.com', 2);