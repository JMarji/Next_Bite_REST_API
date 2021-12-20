-- Create User Registration here

CREATE TABLE Users(
    id uuid NOT NULL UNIQUE,
    Primary KEY (id),
    email TEXT NOT NULL UNIQUE,
    user_name TEXT NOT NULL,
    password
    registered_at timestamptz NOT NULL
    );