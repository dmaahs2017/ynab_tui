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
    account_name              TEXT,
    payee_name                TEXT,
    category_name             TEXT NOT NULL,

    FOREIGN KEY (budget_id)
        REFERENCES budgets (id)
);

CREATE TABLE IF NOT EXISTS accounts (
    id                        TEXT PRIMARY KEY NOT NULL UNIQUE,
    name                      TEXT NOT NULL,
    type                      TEXT NOT NULL,
    on_budget                 INTEGER NOT NULL,
    closed                    INTEGER NOT NULL CHECK(closed IN (0, 1)), --BOOL
    note                      TEXT,
    balance                   INTEGER NOT NULL,
    cleared_balance           INTEGER NOT NULL,
    uncleared_balance         INTEGER NOT NULL,
    transfer_payee_id         TEXT,
    direct_import_linked      INTEGER NOT NULL CHECK(direct_import_linked in (0, 1)), --BOOL
    direct_import_in_error    INTEGER NOT NULL CHECK(direct_import_in_error in (0, 1)), --BOOL

    FOREIGN KEY (transfer_payee_id)
        REFERENCES payees (id)
);

CREATE TABLE IF NOT EXISTS payees (
    id                        TEXT PRIMARY KEY NOT NULL UNIQUE,
    name                      TEXT NOT NULL,
    transfer_account_id       TEXT,
    FOREIGN KEY (transfer_account_id)
        REFERENCES accounts (id)
);


CREATE TABLE IF NOT EXISTS category_groups (
    id                        TEXT PRIMARY KEY NOT NULL UNIQUE,
    name                      TEXT NOT NULL,
    hidden                    INTEGER NOT NULL CHECK(hidden in (0, 1))
);

CREATE TABLE IF NOT EXISTS categories (
    id                         TEXT PRIMARY KEY NOT NULL UNIQUE,
    category_group_id          TEXT NOT NULL,
    name                       TEXT NOT NULL,
    hidden                     INTEGER NOT NULL CHECK(hidden in (0, 1)),
    original_category_group_id TEXT NOT NULL,
    note                       TEXT,
    budgeted                   INTEGER NOT NULL,
    activity                   INTEGER NOT NULL,
    balance                    INTEGER NOT NULL,
    goal_type                  TEXT CHECK(goal_type in ('TB', 'TBD', 'MF', 'NEED', 'DEBT')),
    goal_creation_month        TEXT,
    goal_target                INTEGER,
    goal_month                 TEXT,
    goal_percentage_complete   INTEGER,
    goal_months_to_budget      INTEGER,
    goal_underfunded           INTEGER,
    goal_overall_funded        INTEGER,
    goal_overall_left          INTEGER,

    FOREIGN KEY (category_group_id)
        REFERENCES category_groups (id),
    FOREIGN KEY (original_category_group_id)
        REFERENCES category_groups (id)
);
