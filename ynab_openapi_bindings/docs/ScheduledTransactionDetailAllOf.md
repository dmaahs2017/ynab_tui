# ScheduledTransactionDetailAllOf

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_name** | **String** |  | 
**payee_name** | Option<**String**> |  | [optional]
**category_name** | Option<**String**> | The name of the category.  If a split scheduled transaction, this will be 'Split'. | [optional]
**subtransactions** | [**Vec<crate::models::ScheduledSubTransaction>**](ScheduledSubTransaction.md) | If a split scheduled transaction, the subtransactions. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


