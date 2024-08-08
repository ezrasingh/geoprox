# RemoveKeyBatchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Deleted** | **bool** | If true all keys were removed | 

## Methods

### NewRemoveKeyBatchResponse

`func NewRemoveKeyBatchResponse(deleted bool, ) *RemoveKeyBatchResponse`

NewRemoveKeyBatchResponse instantiates a new RemoveKeyBatchResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRemoveKeyBatchResponseWithDefaults

`func NewRemoveKeyBatchResponseWithDefaults() *RemoveKeyBatchResponse`

NewRemoveKeyBatchResponseWithDefaults instantiates a new RemoveKeyBatchResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeleted

`func (o *RemoveKeyBatchResponse) GetDeleted() bool`

GetDeleted returns the Deleted field if non-nil, zero value otherwise.

### GetDeletedOk

`func (o *RemoveKeyBatchResponse) GetDeletedOk() (*bool, bool)`

GetDeletedOk returns a tuple with the Deleted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeleted

`func (o *RemoveKeyBatchResponse) SetDeleted(v bool)`

SetDeleted sets Deleted field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


