-- Your SQL goes here
CREATE TABLE wallets (
  id UUID PRIMARY KEY,
  balance INTEGER NOT NULL,
  user_id UUID REFERENCES users(id) NOT NULL
);
