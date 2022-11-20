INSERT INTO transactions (
    id,
    budget_id,
    date,
    amount,
    memo,
    account_id,
    payee_id,
    category_id,
    transfer_account_id,
    transfer_transaction_id,
    matched_transaction_id,
    deleted,
    account_name,
    payee_name,
    category_name
) VALUES
(
    :id,
    :budget_id,
    :date,
    :amount,
    :memo,
    :account_id,
    :payee_id,
    :category_id,
    :transfer_account_id,
    :transfer_transaction_id,
    :matched_transaction_id,
    :deleted,
    :account_name,
    :payee_name,
    :category_name
);