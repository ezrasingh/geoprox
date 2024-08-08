# CreateIndexResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Created** | **bool** | If true index was created | 
**Existed** | **bool** | If true index alredy exist | 

## Methods

### NewCreateIndexResponse

`func NewCreateIndexResponse(created bool, existed bool, ) *CreateIndexResponse`

NewCreateIndexResponse instantiates a new CreateIndexResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateIndexResponseWithDefaults

`func NewCreateIndexResponseWithDefaults() *CreateIndexResponse`

NewCreateIndexResponseWithDefaults instantiates a new CreateIndexResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreated

`func (o *CreateIndexResponse) GetCreated() bool`

GetCreated returns the Created field if non-nil, zero value otherwise.

### GetCreatedOk

`func (o *CreateIndexResponse) GetCreatedOk() (*bool, bool)`

GetCreatedOk returns a tuple with the Created field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreated

`func (o *CreateIndexResponse) SetCreated(v bool)`

SetCreated sets Created field to given value.


### GetExisted

`func (o *CreateIndexResponse) GetExisted() bool`

GetExisted returns the Existed field if non-nil, zero value otherwise.

### GetExistedOk

`func (o *CreateIndexResponse) GetExistedOk() (*bool, bool)`

GetExistedOk returns a tuple with the Existed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExisted

`func (o *CreateIndexResponse) SetExisted(v bool)`

SetExisted sets Existed field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


