CREATE TABLE IF NOT EXISTS budgets (
    id                 TEXT PRIMARY KEY NOT NULL UNIQUE,
    name               TEXT NOT NULL,
    last_modified_on   TEXT NOT NULL,
    first_month        TEXT NOT NULL,
    last_month         TEXT NOT NULL,
    date_format        TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS transactions (
    id                        TEXT PRIMARY KEY NOT NULL UNIQUE,
    budget_id                 TEXT NOT NULL,
    date                      TEXT NOT NULL,
    amount                    INTEGER NOT NULL,
    memo                      TEXT,
    account_id                TEXT NOT NULL,
    payee_id                  TEXT,
    category_id               TEXT,
    transfer_account_id       TEXT,
    transfer_transaction_id   TEXT,
    matched_transaction_id    TEXT,
    deleted                   INTEGER NOT NULL, -- bool
    account_name              TEXT,
    payee_name                TEXT,
    category_name             TEXT NOT NULL,
    FOREIGN KEY (budget_id)
        REFERENCES budgets (id)
);
