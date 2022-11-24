update categories
set 
    id = :id,
    category_group_id = :category_group_id,
    name = :name,
    hidden = :hidden,
    original_category_group_id = :original_category_group_id,
    note = :note,
    budgeted = :budgeted,
    activity = :activity,
    balance = :balance,
where
    id = :id;
