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
        REFERENCES budgets (id),
    FOREIGN KEY (account_id)
        REFERENCES accounts (id),
    FOREIGN KEY (payee_id)
        REFERENCES payees (id),
    FOREIGN KEY (category_id)
        REFERENCES categories (id),
    FOREIGN KEY (transfer_account_id)
        REFERENCES accounts (id),
    FOREIGN KEY (transfer_transaction_id)
        REFERENCES transactions (id),
    FOREIGN KEY (matched_transaction_id)
        REFERENCES transactions (id)
);

CREATE TABLE IF NOT EXISTS accounts (
    id                        TEXT PRIMARY KEY NOT NULL UNIQUE,
    name                      TEXT NOT NULL,
    account_type              TEXT NOT NULL CHECK(account_type in ('checking', 'savings', 'cash', 'creditCard', 'lineOfCredit', 'otherAsset', 'otherLiability', 'mortgage', 'autoLoan', 'studentLoan', 'personalLoan', 'medicalDebt', 'otherDebt')),
    on_budget                 INTEGER NOT NULL CHECK(on_budget in (0, 1)), --BOOL
    closed                    INTEGER NOT NULL CHECK(closed in (0, 1)), --BOOL
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

CREATE TABLE IF NOT exists goals (
    id text primary key not null unique,
    type                  TEXT CHECK(type in ('TB', 'TBD', 'MF', 'NEED', 'DEBT')),
    creation_month        TEXT,
    target                INTEGER,
    month                 TEXT,
    percentage_complete   INTEGER,
    months_to_budget      INTEGER,
    underfunded           INTEGER,
    overall_funded        INTEGER,
    overall_left          INTEGER
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
    goal_id                    text,

    FOREIGN KEY (category_group_id)
        REFERENCES category_groups (id),
    FOREIGN KEY (original_category_group_id)
        REFERENCES category_groups (id),
    FOREIGN KEY (goal_id)
        REFERENCES goals (id)
);
