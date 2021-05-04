-- Your SQL goes here
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name text,
    parent_id integer,
    "position" integer,
    created_at text,
    updated_at text,
    ads text,
    deleted_at text,
    image text,
    active integer,
    slug text,
    product_recommendation_id text
);
