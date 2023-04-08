-- Your SQL goes here
CREATE TABLE products (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  price NUMERIC(10, 2) NOT NULL,
  gym_id UUID REFERENCES gyms(id)
);
