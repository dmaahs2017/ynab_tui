pub struct Account {
    id: String,             
    name: String,          
    account_type: String,         
    on_budget : bool,     
    closed         : bool,
    note: String,         
    balance: i64,         
    cleared_balance: i64, 
    uncleared_balance: i64,
    transfer_payee_id: Option<String>         ,
    direct_import_linked: bool,      
    direct_import_in_error: bool,   
}
