-- https://gill.net.in/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email VARCHAR(100) NOT NULL,
  password VARCHAR(64) NOT NULL, --bcrypt hash
  created_at TIMESTAMP NOT NULL
);
