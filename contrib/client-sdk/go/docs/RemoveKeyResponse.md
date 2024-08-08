# RemoveKeyResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Deleted** | **bool** | If true key was removed | 
**Key** | **string** | Object key | 

## Methods

### NewRemoveKeyResponse

`func NewRemoveKeyResponse(deleted bool, key string, ) *RemoveKeyResponse`

NewRemoveKeyResponse instantiates a new RemoveKeyResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRemoveKeyResponseWithDefaults

`func NewRemoveKeyResponseWithDefaults() *RemoveKeyResponse`

NewRemoveKeyResponseWithDefaults instantiates a new RemoveKeyResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeleted

`func (o *RemoveKeyResponse) GetDeleted() bool`

GetDeleted returns the Deleted field if non-nil, zero value otherwise.

### GetDeletedOk

`func (o *RemoveKeyResponse) GetDeletedOk() (*bool, bool)`

GetDeletedOk returns a tuple with the Deleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeleted

`func (o *RemoveKeyResponse) SetDeleted(v bool)`

SetDeleted sets Deleted field to given value.


### GetKey

`func (o *RemoveKeyResponse) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *RemoveKeyResponse) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *RemoveKeyResponse) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


