# \ContactsApi

All URIs are relative to *http://localhost:8090*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_contact**](ContactsApi.md#create_contact) | **POST** /api/contacts | 
[**delete_contact**](ContactsApi.md#delete_contact) | **DELETE** /api/contacts/{contact_id} | 
[**get_contact**](ContactsApi.md#get_contact) | **GET** /api/contacts/{contact_id} | 
[**list_contacts**](ContactsApi.md#list_contacts) | **GET** /api/contacts/list | 
[**update_contact**](ContactsApi.md#update_contact) | **PUT** /api/contacts/{contact_id} | 



## create_contact

> models::Contact create_contact(organization, create_contact_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**create_contact_req_payload** | [**CreateContactReqPayload**](CreateContactReqPayload.md) | JSON request payload to create a new contact | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_contact

> delete_contact(contact_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contacts id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_contact

> models::Contact get_contact(contact_id, organization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contacts id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_contacts

> models::ContactList list_contacts(organization, limit, offset)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization** | **String** | The org id to use for the request | [required] |
**limit** | Option<**i64**> | The number of contacts to return |  |
**offset** | Option<**String**> | The offset to start from |  |

### Return type

[**models::ContactList**](ContactList.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_contact

> models::Contact update_contact(contact_id, organization, update_contact_req_payload)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contact_id** | **String** | The contact id to use for the request | [required] |
**organization** | **String** | The org id to use for the request | [required] |
**update_contact_req_payload** | [**UpdateContactReqPayload**](UpdateContactReqPayload.md) | JSON request payload to update the contact | [required] |

### Return type

[**models::Contact**](Contact.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

