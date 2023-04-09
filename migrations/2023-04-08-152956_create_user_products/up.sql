-- Your SQL goes here
CREATE TABLE user_products (
  id UUID PRIMARY KEY,
  product_id UUID REFERENCES users(id) NOT NULL
  user_id UUID REFERENCES users(id) NOT NULL
);
