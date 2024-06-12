# \DealsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_deal**](DealsApi.md#create_deal) | **POST** /api/deals | 
[**create_deal_resource**](DealsApi.md#create_deal_resource) | **POST** /api/deals/{deal_id}/{resource_type}/{resource_id} | 
[**delete_deal**](DealsApi.md#delete_deal) | **DELETE** /api/deals/{deal_id} | 
[**delete_deal_resource**](DealsApi.md#delete_deal_resource) | **DELETE** /api/deals/{deal_id}/{resource_type}/{resource_id} | 
[**get_deal**](DealsApi.md#get_deal) | **GET** /api/deals/{deal_id} | 
[**list_deal_by_org**](DealsApi.md#list_deal_by_org) | **GET** /api/deals/list/org | 
[**list_deal_resource**](DealsApi.md#list_deal_resource) | **GET** /api/deals/{deal_id}/{resource_type} | 
[**update_deal**](DealsApi.md#update_deal) | **PUT** /api/deals/{deal_id} | 



## create_deal

> models::Deal create_deal(organization, create_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**create_deal_req_payload** | [**CreateDealReqPayload**](CreateDealReqPayload.md) | JSON request payload to create a new deal | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_deal_resource

> models::DealResource create_deal_resource(deal_id, resource_type, resource_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**resource_type** | [**DealResType**](.md) | The resource type to use for the request | [required] |
**resource_id** | **String** | The resource id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

[**models::DealResource**](DealResource.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal

> delete_deal(deal_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_deal_resource

> delete_deal_resource(deal_id, resource_type, resource_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**resource_type** | [**DealResType**](.md) | The resource type to use for the request | [required] |
**resource_id** | **String** | The resource id to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_deal

> models::Deal get_deal(deal_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deal_by_org

> models::ListDealByOrgRespBody list_deal_by_org(organization, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The organization id to use for the request | [required] |
**limit** | Option<**i64**> | The number of records to return |  |
**offset** | Option<**String**> | The UUID of the record to start from |  |

### Return type

[**models::ListDealByOrgRespBody**](ListDealByOrgRespBody.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_deal_resource

> models::DealResourceListWithPagination list_deal_resource(deal_id, resource_type, organization, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**resource_type** | [**DealResType**](.md) | The resource type to use for the request | [required] |
**organization** | **String** | The organization id to use for the request | [required] |
**limit** | Option<**i64**> | The number of records to return |  |
**offset** | Option<**String**> | The UUID of the record to start from |  |

### Return type

[**models::DealResourceListWithPagination**](DealResourceListWithPagination.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_deal

> models::Deal update_deal(deal_id, organization, update_deal_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deal_id** | **String** | The deal id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |
**update_deal_req_payload** | [**UpdateDealReqPayload**](UpdateDealReqPayload.md) | JSON request payload to update the deal | [required] |

### Return type

[**models::Deal**](Deal.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

