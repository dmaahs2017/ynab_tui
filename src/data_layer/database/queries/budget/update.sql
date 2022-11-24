UPDATE budgets
SET name = :name,
    last_modified_on = :last_modified_on,
    first_month = :first_month,
    last_month = :last_month,
    date_format = :date_format
WHERE
    id = :id;
