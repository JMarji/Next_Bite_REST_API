-- Create User Registration here

CREATE TABLE Users(
    id uuid NOT NULL UNIQUE,
    Primary KEY (id),
    email TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    registered_at timestamptz NOT NULL
    );