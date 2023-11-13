CREATE TABLE IF NOT EXISTS todos (
    id UUID PRIMARY KEY,
    name VARCHAR(120) NOT NULL,
    completed BOOLEAN NOT NULL CHECK (completed in (0, 1)),
    inserted_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL
);
