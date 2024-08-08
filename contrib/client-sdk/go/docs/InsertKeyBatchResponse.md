# InsertKeyBatchResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Errors** | **map[string]string** | Contains information about which keys failed to be inserted and the associated error details. | 
**Results** | **map[string]string** | Object keys that have been inserted in the index and their geohash. | 

## Methods

### NewInsertKeyBatchResponse

`func NewInsertKeyBatchResponse(errors map[string]string, results map[string]string, ) *InsertKeyBatchResponse`

NewInsertKeyBatchResponse instantiates a new InsertKeyBatchResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInsertKeyBatchResponseWithDefaults

`func NewInsertKeyBatchResponseWithDefaults() *InsertKeyBatchResponse`

NewInsertKeyBatchResponseWithDefaults instantiates a new InsertKeyBatchResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetErrors

`func (o *InsertKeyBatchResponse) GetErrors() map[string]string`

GetErrors returns the Errors field if non-nil, zero value otherwise.

### GetErrorsOk

`func (o *InsertKeyBatchResponse) GetErrorsOk() (*map[string]string, bool)`

GetErrorsOk returns a tuple with the Errors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrors

`func (o *InsertKeyBatchResponse) SetErrors(v map[string]string)`

SetErrors sets Errors field to given value.


### GetResults

`func (o *InsertKeyBatchResponse) GetResults() map[string]string`

GetResults returns the Results field if non-nil, zero value otherwise.

### GetResultsOk

`func (o *InsertKeyBatchResponse) GetResultsOk() (*map[string]string, bool)`

GetResultsOk returns a tuple with the Results field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResults

`func (o *InsertKeyBatchResponse) SetResults(v map[string]string)`

SetResults sets Results field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


