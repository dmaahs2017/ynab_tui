# \AccountsApi

All URIs are relative to *https://api.ynab.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_account**](AccountsApi.md#create_account) | **POST** /budgets/{budget_id}/accounts | Create a new account
[**get_account_by_id**](AccountsApi.md#get_account_by_id) | **GET** /budgets/{budget_id}/accounts/{account_id} | Single account
[**get_accounts**](AccountsApi.md#get_accounts) | **GET** /budgets/{budget_id}/accounts | Account list



## create_account

> crate::models::AccountResponse create_account(budget_id, data)
Create a new account

Creates a new account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget (\"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget) | [required] |
**data** | [**PostAccountWrapper**](PostAccountWrapper.md) | The account to create. | [required] |

### Return type

[**crate::models::AccountResponse**](AccountResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_by_id

> crate::models::AccountResponse get_account_by_id(budget_id, account_id)
Single account

Returns a single account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**account_id** | **uuid::Uuid** | The id of the account | [required] |

### Return type

[**crate::models::AccountResponse**](AccountResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_accounts

> crate::models::AccountsResponse get_accounts(budget_id, last_knowledge_of_server)
Account list

Returns all accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**budget_id** | **String** | The id of the budget. \"last-used\" can be used to specify the last used budget and \"default\" can be used if default budget selection is enabled (see: https://api.ynab.com/#oauth-default-budget). | [required] |
**last_knowledge_of_server** | Option<**i64**> | The starting server knowledge.  If provided, only entities that have changed since `last_knowledge_of_server` will be included. |  |

### Return type

[**crate::models::AccountsResponse**](AccountsResponse.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

