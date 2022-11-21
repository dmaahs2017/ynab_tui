SELECT * 
FROM transactions 
WHERE budget_id = :budget_id
ORDER BY date DESC;
