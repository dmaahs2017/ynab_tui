CREATE TABLE IF NOT EXISTS budgets (
    id                 TEXT PRIMARY KEY NOT NULL UNIQUE,
    name               TEXT NOT NULL,
    last_modified_on   TEXT NOT NULL,
    first_month        TEXT NOT NULL,
    last_month         TEXT NOT NULL,
    date_format        TEXT NOT NULL
);
