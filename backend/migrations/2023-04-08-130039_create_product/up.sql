-- Your SQL goes here
CREATE TABLE products (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL,
  price INTEGER NOT NULL,
  gym_id UUID REFERENCES gyms(id) NOT NULL
);
