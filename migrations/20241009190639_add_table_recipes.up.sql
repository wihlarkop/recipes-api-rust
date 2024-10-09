CREATE TABLE IF NOT EXISTS recipes (
    uuid UUID PRIMARY KEY,
    name VARCHAR (255) NOT NULL,
    description TEXT,
    ingredients TEXT NOT NULL,
    instructions TEXT NOT NULL,
    prep_time INT,
    cook_time INT,
    servings INT,
    difficulty_level VARCHAR (50),
    created_by varchar(255) NOT NULL,
    updated_by varchar(255),
    deleted_by varchar(255),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ,
    deleted_at TIMESTAMPTZ
);
