UPDATE transactions
SET id = :id,
    date = :date,
    amount = :amount,
    memo = :memo,
    account_id = :account_id,
    payee_id = :payee_id,
    category_id = :category_id,
    transfer_account_id = :transfer_account_id,
    transfer_transaction_id = :transfer_transaction_id,
    matched_transaction_id = :matched_transaction_id,
WHERE
    id = :id;
