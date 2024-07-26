-- ➜  zero2prod git:(main) ✗ export DATABASE_URL=postgres://postgres:password@127.0.0.1:5432/newsletter
-- ➜  zero2prod git:(main) ✗ sqlx migrate add create_subscriptions_table
-- Add migration script here
create table subscriptions(
    id uuid not null primary key,
    name text not null,
    email text not null unique,
    subscribed_at timestamptz not null
);
