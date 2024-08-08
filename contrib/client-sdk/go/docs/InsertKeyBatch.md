# InsertKeyBatch

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Keys** | [**[]InsertKey**](InsertKey.md) | Object key | 
**PreserveOrder** | **bool** |  | 

## Methods

### NewInsertKeyBatch

`func NewInsertKeyBatch(keys []InsertKey, preserveOrder bool, ) *InsertKeyBatch`

NewInsertKeyBatch instantiates a new InsertKeyBatch object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInsertKeyBatchWithDefaults

`func NewInsertKeyBatchWithDefaults() *InsertKeyBatch`

NewInsertKeyBatchWithDefaults instantiates a new InsertKeyBatch object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKeys

`func (o *InsertKeyBatch) GetKeys() []InsertKey`

GetKeys returns the Keys field if non-nil, zero value otherwise.

### GetKeysOk

`func (o *InsertKeyBatch) GetKeysOk() (*[]InsertKey, bool)`

GetKeysOk returns a tuple with the Keys field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeys

`func (o *InsertKeyBatch) SetKeys(v []InsertKey)`

SetKeys sets Keys field to given value.


### GetPreserveOrder

`func (o *InsertKeyBatch) GetPreserveOrder() bool`

GetPreserveOrder returns the PreserveOrder field if non-nil, zero value otherwise.

### GetPreserveOrderOk

`func (o *InsertKeyBatch) GetPreserveOrderOk() (*bool, bool)`

GetPreserveOrderOk returns a tuple with the PreserveOrder field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreserveOrder

`func (o *InsertKeyBatch) SetPreserveOrder(v bool)`

SetPreserveOrder sets PreserveOrder field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


