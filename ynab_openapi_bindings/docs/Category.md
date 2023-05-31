# Category

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**category_group_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**category_group_name** | Option<**String**> |  | [optional]
**name** | **String** |  | 
**hidden** | **bool** | Whether or not the category is hidden | 
**original_category_group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | DEPRECATED: No longer used.  Value will always be null. | [optional]
**note** | Option<**String**> |  | [optional]
**budgeted** | **i64** | Budgeted amount in milliunits format | 
**activity** | **i64** | Activity amount in milliunits format | 
**balance** | **i64** | Balance in milliunits format | 
**goal_type** | Option<**String**> | The type of goal, if the category has a goal (TB='Target Category Balance', TBD='Target Category Balance by Date', MF='Monthly Funding', NEED='Plan Your Spending') | [optional]
**goal_day** | Option<**i32**> | The day of the goal | [optional]
**goal_cadence** | Option<**i32**> | The goal cadence | [optional]
**goal_cadence_frequency** | Option<**i32**> | The goal cadence frequency | [optional]
**goal_creation_month** | Option<[**String**](string.md)> | The month a goal was created | [optional]
**goal_target** | Option<**i64**> | The goal target amount in milliunits | [optional]
**goal_target_month** | Option<[**String**](string.md)> | The original target month for the goal to be completed.  Only some goal types specify this date. | [optional]
**goal_percentage_complete** | Option<**i32**> | The percentage completion of the goal | [optional]
**goal_months_to_budget** | Option<**i32**> | The number of months, including the current month, left in the current goal period. | [optional]
**goal_under_funded** | Option<**i64**> | The amount of funding still needed in the current month to stay on track towards completing the goal within the current goal period.  This amount will generally correspond to the 'Underfunded' amount in the web and mobile clients except when viewing a category with a Needed for Spending Goal in a future month.  The web and mobile clients will ignore any funding from a prior goal period when viewing category with a Needed for Spending Goal in a future month. | [optional]
**goal_overall_funded** | Option<**i64**> | The total amount funded towards the goal within the current goal period. | [optional]
**goal_overall_left** | Option<**i64**> | The amount of funding still needed to complete the goal within the current goal period. | [optional]
**deleted** | **bool** | Whether or not the category has been deleted.  Deleted categories will only be included in delta requests. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


