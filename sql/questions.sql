CREATE TABLE IF NOT EXISTS questions (
    -- We let PostgreSQL create the IDs for us.
    id serial PRIMARY KEY,
    title VARCHAR (255) NOT NULL,
    content TEXT NOT NULL,
    tags TEXT [],
    -- It's always wise to have a timestamp attached to entries,
    -- and we tell PostgreSQL to create one for us by default.
    created_on TIMESTAMP NOT NULL DEFAULT NOW()
);